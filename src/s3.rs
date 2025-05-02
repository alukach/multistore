use crate::conversion::S3ObjectMeta;
use crate::data_source::DataSourceRegistry;
use futures_util::TryStreamExt;
use futures_util::stream::iter;
use log::error;
use s3s::dto;
use s3s::{S3, S3Request, S3Response, S3Result};
use s3s::{S3Error, S3ErrorCode};
use std::pin::Pin;

#[derive(Clone)]
pub struct S3Interface<T: DataSourceRegistry + Send + Sync + 'static> {
    source: T,
}

impl<T: DataSourceRegistry + Send + Sync> S3Interface<T> {
    pub fn new(source: T) -> Self {
        Self { source }
    }
}

// Shared pagination utility
async fn paginate_items<T, F, R>(
    items: Pin<Box<dyn futures_util::Stream<Item = Result<T, S3Error>> + Send>>,
    max_items: usize,
    transform: F,
) -> S3Result<(Vec<R>, bool)>
where
    F: Fn(T) -> R,
{
    let mut results = Vec::with_capacity(max_items);
    let mut stream = items;

    while let Some(item) = stream.try_next().await? {
        if results.len() >= max_items {
            break;
        }
        results.push(transform(item));
    }

    let is_truncated = results.len() >= max_items;
    Ok((results, is_truncated))
}

// TODO: When an object is read, we should emit metrics
#[async_trait::async_trait]
impl<T: DataSourceRegistry + Send + Sync + Clone + 'static> S3 for S3Interface<T> {
    async fn list_buckets(
        &self,
        req: S3Request<dto::ListBucketsInput>,
    ) -> S3Result<S3Response<dto::ListBucketsOutput>> {
        let access_key = req.credentials.map(|c| c.access_key.clone());
        let max_buckets = 1000; // Default max buckets per request

        // Get all data sources and convert them to a stream
        // TODO: Should list_data_sources return a stream? Should it support pagination natively?
        let data_sources = self.source.list_data_sources(access_key.as_ref()).await;
        let stream = iter(data_sources.into_iter().map(Ok::<_, S3Error>));

        let (buckets, is_truncated) =
            paginate_items(Box::pin(stream), max_buckets, |source| source.into()).await?;

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
        let max_keys = req.input.max_keys.unwrap_or(1000) as usize;
        let start_after = req
            .input
            .start_after
            .map(|s| object_store::path::Path::from(s))
            .unwrap_or(object_store::path::Path::from("/".to_string()));

        let (object_store, path) =
            self.source
                .get_object_store(&bucket_name)
                .await
                .map_err(|e| {
                    error!("Failed to get object store: {:?}", e);
                    S3Error::new(S3ErrorCode::InternalError)
                })?;

        let stream = object_store
            .list_with_offset(Some(&path), &start_after)
            .map_err(|e| {
                error!("Error listing objects: {:?}", e);
                S3Error::new(S3ErrorCode::InternalError)
            });

        let (objects, is_truncated) = paginate_items(Box::pin(stream), max_keys, |meta| {
            S3ObjectMeta::from(meta).into()
        })
        .await?;

        Ok(S3Response::new(dto::ListObjectsV2Output {
            contents: Some(objects),
            is_truncated: Some(is_truncated),
            max_keys: Some(max_keys as i32),
            ..Default::default()
        }))
    }
}
