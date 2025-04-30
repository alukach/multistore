use std::sync::Arc;

use crate::database::BucketRegistry;
use s3s::dto;
use s3s::{S3, S3Request, S3Response, S3Result};

#[derive(Clone)]
pub struct S3Interface {
    source: Arc<dyn BucketRegistry + Send + Sync>,
}

impl S3Interface {
    pub fn new(source: Arc<dyn BucketRegistry + Send + Sync>) -> Self {
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
        let buckets = self.source.list_buckets().await;
        let output = dto::ListBucketsOutput {
            buckets: Some(buckets),
            owner: None,
            ..Default::default()
        };
        Ok(S3Response::new(output))
    }
}
