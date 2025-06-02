use crate::error::{Error, Result};
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

impl DataSource {
    pub fn as_object_store(self, prefix: Option<String>) -> Result<(Box<dyn ObjectStore>, Path)> {
        let (object_store, root_prefix) = self.try_into()?;
        let mut full_path = format!("{}/{}", root_prefix.to_string(), prefix.unwrap_or_default());
        if full_path.ends_with("/") {
            full_path = full_path.strip_suffix("/").unwrap().to_string();
        }
        Ok((object_store, Path::from(full_path)))
    }
}

/// A data source registery is the tooling to track the existence of data sources (ie
/// various Object Storage backends).
#[async_trait::async_trait]
pub trait DataSourceRegistry {
    async fn list_data_sources(
        &self,
        access_key: Option<&String>,
        input: dto::ListBucketsInput,
    ) -> Vec<DataSource>;

    async fn get_data_source(&self, name: &str) -> Result<DataSource>;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_source() -> DataSource {
        DataSource {
            name: "test-bucket".to_string(),
            region: "us-east-1".to_string(),
            url: "s3://test-bucket".to_string(),
            creation_date: None,
            credentials: HashMap::new(),
        }
    }

    #[tokio::test]
    async fn test_as_object_store_without_prefix() {
        let source = create_test_source();
        let (_, path) = source.as_object_store(None).unwrap();
        assert_eq!(path.to_string(), "");
    }

    #[tokio::test]
    async fn test_as_object_store_with_prefix() {
        let source = create_test_source();
        let (_, path) = source
            .as_object_store(Some("test/prefix".to_string()))
            .unwrap();
        assert_eq!(path.to_string(), "test/prefix");
    }

    #[tokio::test]
    async fn test_as_object_store_with_empty_prefix() {
        let source = create_test_source();
        let (_, path) = source.as_object_store(Some("".to_string())).unwrap();
        assert_eq!(path.to_string(), "");
    }
}
