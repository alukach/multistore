use crate::conversion::{S3ObjectMeta, Timestamp, parse_etag};
use crate::data_source::DataSourceRegistry;
use crate::error::Error;
use crate::stream::SyncStream;
use futures_util::TryStreamExt;
use object_store::path::Path;
use object_store::{GetOptions, ObjectStore};
use s3s::dto;
use s3s::dto::StreamingBlob;
use s3s::{S3, S3Request, S3Response, S3Result};
use tracing::{debug, instrument};

#[derive(Clone)]
pub struct S3Interface<T: DataSourceRegistry + Send + Sync + 'static> {
    registry: T,
}

impl<T: DataSourceRegistry + Send + Sync> S3Interface<T> {
    pub fn new(registry: T) -> Self {
        Self { registry }
    }
}

// TODO: When an object is read, we should emit metrics
#[async_trait::async_trait]
impl<T: DataSourceRegistry + Send + Sync + Clone + 'static> S3 for S3Interface<T> {
    #[instrument(skip(self, req), fields(access_key = ?req.credentials.as_ref().map(|c| &c.access_key)))]
    async fn list_buckets(
        &self,
        req: S3Request<dto::ListBucketsInput>,
    ) -> S3Result<S3Response<dto::ListBucketsOutput>> {
        debug!("Listing buckets");
        let access_key = req.credentials.map(|c| c.access_key.clone());
        // TODO: Support req.input.continuation_token,
        let buckets: Vec<_> = self
            .registry
            .list_data_sources(access_key.as_ref(), req.input)
            .await
            .into_iter()
            .map(Into::into)
            .collect();

        debug!(bucket_count = buckets.len(), "Listed buckets successfully");
        Ok(S3Response::new(dto::ListBucketsOutput {
            buckets: Some(buckets),
            owner: None,
            ..Default::default()
        }))
    }

    #[instrument(skip(self, req), fields(bucket = %req.input.bucket, prefix = ?req.input.prefix))]
    async fn list_objects_v2(
        &self,
        req: S3Request<dto::ListObjectsV2Input>,
    ) -> S3Result<S3Response<dto::ListObjectsV2Output>> {
        debug!("Listing objects");
        let source = self.registry.get_data_source(&req.input.bucket).await?;
        let (object_store, prefix) = source.as_object_store(req.input.prefix.clone())?;
        debug!(%prefix, "Resolved object store path");

        let max_keys = req.input.max_keys.unwrap_or(1000) as usize;
        let start_after = req
            .input
            .start_after
            .map(Path::from)
            .unwrap_or(Path::from("/"));

        let mut response = dto::ListObjectsV2Output {
            max_keys: Some(max_keys.try_into().unwrap_or(i32::MAX)),
            ..Default::default()
        };

        if req.input.delimiter.is_some() {
            let list_result = object_store
                .list_with_delimiter(Some(&prefix))
                .await
                .map_err(Error::from)?;

            let objects: Vec<_> = list_result
                .objects
                .into_iter()
                .map(|obj| S3ObjectMeta::from(obj).into())
                .collect();

            let common_prefixes: Vec<_> = list_result
                .common_prefixes
                .into_iter()
                .map(|prefix| dto::CommonPrefix {
                    prefix: Some(format!("{}/", prefix)),
                })
                .collect();

            let total_items = objects.len() + common_prefixes.len();
            debug!(
                objects = objects.len(),
                prefixes = common_prefixes.len(),
                "Listed with delimiter"
            );
            response.contents = Some(objects);
            response.common_prefixes = Some(common_prefixes);
            response.is_truncated = Some(total_items > max_keys);
        } else {
            let mut objects = Vec::with_capacity(max_keys);
            let mut stream = object_store.list_with_offset(Some(&prefix), &start_after);

            while let Some(result) = stream.try_next().await.map_err(Error::from)? {
                objects.push(S3ObjectMeta::from(result).into());
                if objects.len() >= max_keys {
                    response.contents = Some(objects);
                    response.is_truncated = Some(true);
                    return Ok(S3Response::new(response));
                }
            }
            debug!(objects = objects.len(), truncated = false, "Listed objects");
            response.contents = Some(objects);
            response.is_truncated = Some(false);
        }

        Ok(S3Response::new(response))
    }

    #[instrument(skip(self, req), fields(bucket = %req.input.bucket, key = %req.input.key))]
    async fn head_object(
        &self,
        req: S3Request<dto::HeadObjectInput>,
    ) -> S3Result<S3Response<dto::HeadObjectOutput>> {
        debug!("Getting object metadata");
        let source = self.registry.get_data_source(&req.input.bucket).await?;
        let (object_store, key) = source.as_object_store(Some(req.input.key))?;
        let object = object_store.head(&key).await.map_err(Error::from)?;
        debug!(size = object.size, "Retrieved object metadata");
        Ok(S3Response::new(dto::HeadObjectOutput {
            content_length: Some(object.size as i64),
            version_id: object.version,
            e_tag: object.e_tag.map(|s| parse_etag(s)),
            last_modified: Some(Timestamp::from(object.last_modified).into()),
            ..Default::default()
        }))
    }

    #[instrument(skip(self, req), fields(bucket = %req.input.bucket, key = %req.input.key, range = ?req.input.range))]
    async fn get_object(
        &self,
        req: S3Request<dto::GetObjectInput>,
    ) -> S3Result<S3Response<dto::GetObjectOutput>> {
        debug!("Getting object");
        let source = self.registry.get_data_source(&req.input.bucket).await?;
        let (object_store, key) = source.as_object_store(Some(req.input.key))?;
        let range = match req.input.range {
            Some(r) => match r {
                dto::Range::Int { first, last } => match last {
                    Some(last) => Some(object_store::GetRange::from(first..last)),
                    None => Some(object_store::GetRange::from(first..)),
                },
                dto::Range::Suffix { length } => Some(object_store::GetRange::from(length..)),
            },
            None => None,
        };
        let opts = GetOptions {
            range,
            ..GetOptions::default()
        };
        let object = object_store
            .get_opts(&key, opts)
            .await
            .map_err(Error::from)?;

        let meta = object.meta.clone();
        debug!(size = meta.size, "Retrieved object successfully");
        let raw_stream = object.into_stream().map_err(Error::from);

        Ok(S3Response::new(dto::GetObjectOutput {
            body: Some(StreamingBlob::wrap(Box::pin(SyncStream(raw_stream)))),
            content_length: Some(meta.size as i64),
            version_id: meta.version,
            e_tag: meta.e_tag.map(|s| parse_etag(s)),
            last_modified: Some(Timestamp::from(meta.last_modified).into()),
            ..Default::default()
        }))
    }
}
