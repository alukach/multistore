use crate::conversion::S3ObjectMeta;
use crate::data_source::DataSourceRegistry;
use futures_util::TryStreamExt;
use s3s::dto;
use s3s::{S3, S3Request, S3Response, S3Result};
use s3s::{S3Error, S3ErrorCode};
use std::sync::Arc;

#[derive(Clone)]
pub struct S3Interface {
    source: Arc<dyn DataSourceRegistry + Send + Sync>,
}

impl S3Interface {
    pub fn new(source: Arc<dyn DataSourceRegistry + Send + Sync>) -> Self {
        Self { source }
    }
}

// TODO: When an object is read, we should emit metrics
#[async_trait::async_trait]
impl S3 for S3Interface {
    async fn list_buckets(
        &self,
        req: S3Request<dto::ListBucketsInput>,
    ) -> S3Result<S3Response<dto::ListBucketsOutput>> {
        let access_key = req.credentials.map(|c| c.access_key.clone());

        let buckets = self
            .source
            .list_data_sources(access_key.as_ref())
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
        // TODO: Support pagination
        let bucket_name = req.input.bucket;
        let (object_store, path) = match self.source.get_object_store(&bucket_name).await {
            Ok(object_store) => object_store,
            Err(e) => {
                println!("Failed to get object store: {:?}", e);
                return Err(S3Error::new(S3ErrorCode::InternalError));
            }
        };

        let objects = object_store
            .list(Some(&path))
            .map_ok(|f| S3ObjectMeta::from(f).into())
            .try_collect()
            .await
            .unwrap();

        Ok(S3Response::new(dto::ListObjectsV2Output {
            contents: Some(objects),
            ..Default::default()
        }))
    }
}
