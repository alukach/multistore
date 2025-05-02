use crate::conversion::S3ObjectMeta;
use crate::data_source::DataSourceRegistry;
use futures_util::TryStreamExt;
use s3s::dto;
use s3s::{S3, S3Request, S3Response, S3Result};
use s3s::{S3Error, S3ErrorCode};

#[derive(Clone)]
pub struct S3Interface<T: DataSourceRegistry + Send + Sync + 'static> {
    source: T,
}

impl<T: DataSourceRegistry + Send + Sync> S3Interface<T> {
    pub fn new(source: T) -> Self {
        Self { source }
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
        let bucket_name = req.input.bucket;
        let max_keys = req.input.max_keys.unwrap_or(100) as usize;
        let start_after = req
            .input
            .start_after
            .map(|s| object_store::path::Path::from(s))
            .unwrap_or(object_store::path::Path::from("/".to_string()));

        let (object_store, path) = match self.source.get_object_store(&bucket_name).await {
            Ok(object_store) => object_store,
            Err(e) => {
                println!("Failed to get object store: {:?}", e);
                return Err(S3Error::new(S3ErrorCode::InternalError));
            }
        };

        let mut objects = Vec::with_capacity(max_keys);
        let mut is_truncated = false;

        // List objects with pagination
        let mut stream = object_store.list_with_offset(Some(&path), &start_after);
        let mut count = 0;

        while let Some(result) = stream.try_next().await.map_err(|e| {
            println!("Error listing objects: {:?}", e);
            S3Error::new(S3ErrorCode::InternalError)
        })? {
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
}
