use crate::conversion::{S3ObjectMeta, Timestamp};
use crate::data_source::DataSourceRegistry;
use crate::error::Error;
use crate::stream::SyncStream;
use futures_util::TryStreamExt;
use object_store::path::Path;
use s3s::dto;
use s3s::dto::StreamingBlob;
use s3s::{S3, S3Request, S3Response, S3Result};
use std::collections::HashMap;

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
    async fn list_buckets(
        &self,
        req: S3Request<dto::ListBucketsInput>,
    ) -> S3Result<S3Response<dto::ListBucketsOutput>> {
        let access_key = req.credentials.map(|c| c.access_key.clone());
        // TODO: Support req.input.continuation_token,
        let buckets = self
            .registry
            .list_data_sources(access_key.as_ref(), req.input)
            .await
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(S3Response::new(dto::ListBucketsOutput {
            buckets: Some(buckets),
            owner: None,
            ..Default::default()
        }))
    }

    async fn list_objects_v2(
        &self,
        req: S3Request<dto::ListObjectsV2Input>,
    ) -> S3Result<S3Response<dto::ListObjectsV2Output>> {
        let bucket_name = req.input.bucket;
        let source = self.registry.get_data_source(&bucket_name).await?;
        let (object_store, source_prefix) = source.as_object_store(req.input.prefix)?;

        let max_keys = req.input.max_keys.unwrap_or(100) as usize;
        let start_after = req
            .input
            .start_after
            .map(Path::from)
            .unwrap_or(Path::from("/"));

        // List objects with pagination
        let mut objects = Vec::with_capacity(max_keys);
        let mut stream = object_store.list_with_offset(Some(&source_prefix), &start_after);
        let mut count = 0;
        let mut is_truncated = false;

        while let Some(result) = stream.try_next().await.map_err(Error::from)? {
            let obj: dto::Object = S3ObjectMeta::from(result).into();

            // Add the object to our results
            objects.push(obj);
            count += 1;

            // Check if we've reached max_keys
            if count >= max_keys {
                is_truncated = true;
                break;
            }
        }

        Ok(S3Response::new(dto::ListObjectsV2Output {
            contents: Some(objects),
            is_truncated: Some(is_truncated),
            max_keys: Some(max_keys as i32),
            ..Default::default()
        }))
    }

    async fn head_object(
        &self,
        req: S3Request<dto::HeadObjectInput>,
    ) -> S3Result<S3Response<dto::HeadObjectOutput>> {
        let bucket_name = req.input.bucket;
        let source = self.registry.get_data_source(&bucket_name).await?;
        let (object_store, source_prefix) = source.as_object_store(Some(req.input.key))?;
        let object = object_store
            .head(&source_prefix)
            .await
            .map_err(Error::from)?;
        Ok(S3Response::new(dto::HeadObjectOutput {
            content_length: Some(object.size as i64),
            version_id: object.version,
            e_tag: object.e_tag,
            last_modified: Some(Timestamp::from(object.last_modified).into()),
            ..Default::default()
        }))
    }

    async fn get_object(
        &self,
        req: S3Request<dto::GetObjectInput>,
    ) -> S3Result<S3Response<dto::GetObjectOutput>> {
        let bucket_name = req.input.bucket;
        let source = self.registry.get_data_source(&bucket_name).await?;
        let (object_store, source_prefix) = source.as_object_store(Some(req.input.key))?;
        let object = object_store
            .get(&source_prefix)
            .await
            .map_err(Error::from)?;

        let meta = object.meta.clone();
        let raw_stream = object.into_stream().map_err(Error::from);

        Ok(S3Response::new(dto::GetObjectOutput {
            body: Some(StreamingBlob::wrap(Box::pin(SyncStream(raw_stream)))),
            content_length: Some(meta.size as i64),
            version_id: meta.version,
            e_tag: meta.e_tag,
            last_modified: Some(Timestamp::from(meta.last_modified).into()),
            ..Default::default()
        }))
    }
}
