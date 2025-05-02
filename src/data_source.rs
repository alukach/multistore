use crate::error::Error;
use object_store::path::Path;
use object_store::{ObjectStore, parse_url_opts};
use s3s::dto;
use std::collections::HashMap;
use url::Url;
pub mod yaml_db;

#[derive(Debug, Clone)]
pub struct DataSource {
    name: String,
    region: String,
    url: String,
    creation_date: Option<dto::Timestamp>,
    credentials: HashMap<String, String>,
}

impl DataSource {}

/// A data source registery is the tooling to track the existence of data sources (ie
/// various Object Storage backends).
#[async_trait::async_trait]
pub trait DataSourceRegistry {
    async fn list_data_sources(&self, access_key: Option<&String>) -> Vec<DataSource>;

    async fn get_object_store(
        &self,
        bucket_name: &str,
    ) -> Result<(Box<dyn ObjectStore>, Path), Error>;
}

impl From<DataSource> for dto::Bucket {
    fn from(source: DataSource) -> Self {
        Self {
            name: Some(source.name),
            bucket_region: Some(source.region),
            creation_date: source.creation_date,
        }
    }
}

impl TryFrom<DataSource> for (Box<dyn ObjectStore>, Path) {
    type Error = Error;

    fn try_from(source: DataSource) -> Result<Self, Self::Error> {
        let url = Url::parse(&source.url).unwrap();
        let mut options = vec![("region", source.region.clone())];

        // Dump all credentials from the hashmap as key-value pairs
        options.extend(
            source
                .credentials
                .iter()
                .map(|(k, v)| (k.as_str(), v.clone())),
        );

        let (object_store, path) = parse_url_opts(&url, options).unwrap();
        Ok((object_store, path))
    }
}
