use crate::error::Error;
use object_store::{ObjectStore, parse_url_opts};
use s3s::dto;
use std::sync::Arc;
use url::Url;
pub mod yaml_db;

#[derive(Debug, Clone)]
pub struct DataSource {
    name: String,
    region: String,
    url: String,
    creation_date: Option<dto::Timestamp>,
}

impl DataSource {}

/// A data source registery is the tooling to track the existence of data sources (ie
/// various Object Storage backends).
#[async_trait::async_trait]
pub trait DataSourceRegistry {
    async fn list_buckets(&self, access_key: Option<&String>) -> Vec<dto::Bucket>;

    async fn get_object_store(&self, bucket_name: &str) -> Result<Arc<dyn ObjectStore>, Error>;
}

impl Into<dto::Bucket> for DataSource {
    fn into(self) -> dto::Bucket {
        dto::Bucket {
            name: Some(self.name),
            bucket_region: Some(self.region),
            creation_date: self.creation_date,
        }
    }
}

impl TryFrom<&DataSource> for Arc<dyn ObjectStore> {
    type Error = Error;

    fn try_from(source: &DataSource) -> Result<Self, Self::Error> {
        let url = Url::parse(&source.url).unwrap();
        let options = [
            ("region", source.region.clone()),
            // ("skip_signature", source.public.to_string()),
            // TODO: get temporary credentials from assumed role
            // ("access_key_id", "TODO: get from env".to_string()),
            // ("secret_access_key", "TODO: get from env".to_string()),
        ];
        let (object_store, _path) = parse_url_opts(&url, options).unwrap();
        Ok(Arc::new(object_store))
    }
}
