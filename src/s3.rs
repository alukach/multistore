use crate::data_source::DataSourceRegistry;
use futures_util::TryStreamExt;
use object_store::path::Path;
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
        req: S3Request<dto::ListObjectsV2Input>,
    ) -> S3Result<S3Response<dto::ListObjectsV2Output>> {
        let bucket_name = req.input.bucket;
        let object_store = match self.source.get_object_store(&bucket_name).await {
            Ok(object_store) => object_store,
            Err(e) => {
                println!("Failed to get object store: {:?}", e);
                return Err(S3Error::new(S3ErrorCode::UnexpectedContent));
            }
        };

        let Ok(path) = Path::from_url_path(req.input.prefix.as_ref().unwrap()) else {
            return Err(S3Error::new(S3ErrorCode::UnexpectedContent));
        };

        let objects = object_store
            .list(Some(&path))
            .map_ok(|f| dto::Object {
                key: Some(f.location.to_string()),
                size: Some(f.size as i64),
                last_modified: Some(dto::Timestamp::from(std::time::SystemTime::from(
                    f.last_modified,
                ))),
                e_tag: f.e_tag,
                // version: f.version,
                ..Default::default()
            })
            .try_collect()
            .await
            .unwrap();

        Ok(S3Response::new(dto::ListObjectsV2Output {
            contents: Some(objects),
            ..Default::default()
        }))
    }
}
