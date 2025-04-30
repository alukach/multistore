use s3s::dto;
use s3s::{S3, S3Request, S3Response, S3Result};

#[derive(Clone)]
pub struct ObjectStore {}

#[async_trait::async_trait]
impl S3 for ObjectStore {
    async fn list_buckets(
        &self,
        _req: S3Request<dto::ListBucketsInput>,
    ) -> S3Result<S3Response<dto::ListBucketsOutput>> {
        let mut buckets: Vec<dto::Bucket> = Vec::new();

        // TODO: Get buckets from storage
        buckets.push(dto::Bucket {
            name: Some("foo".to_string()),
            creation_date: Some(dto::Timestamp::from(std::time::SystemTime::now())),
            bucket_region: None,
        });
        buckets.push(dto::Bucket {
            name: Some("bar".to_string()),
            creation_date: Some(dto::Timestamp::from(std::time::SystemTime::now())),
            bucket_region: None,
        });

        let output = dto::ListBucketsOutput {
            buckets: Some(buckets),
            owner: None,
            ..Default::default()
        };
        Ok(S3Response::new(output))
    }
}