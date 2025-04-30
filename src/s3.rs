use std::sync::Arc;

use crate::data_source::DataSourceRegistry;
use s3s::dto;
use s3s::{S3, S3Request, S3Response, S3Result};

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
        _req: S3Request<dto::ListBucketsInput>,
    ) -> S3Result<S3Response<dto::ListBucketsOutput>> {
        let access_key = _req.credentials.map(|c| c.access_key.clone());
        let buckets = self.source.list_buckets(access_key.as_ref()).await;
        let output = dto::ListBucketsOutput {
            buckets: Some(buckets),
            owner: None,
            ..Default::default()
        };
        Ok(S3Response::new(output))
    }

    async fn list_objects_v2(
        &self,
        _req: S3Request<dto::ListObjectsV2Input>,
    ) -> S3Result<S3Response<dto::ListObjectsV2Output>> {
        todo!()
    }
}
