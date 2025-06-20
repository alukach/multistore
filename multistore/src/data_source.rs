use crate::error::{Error, Result};
use object_store::client::HttpConnector;
use object_store::path::Path;
use object_store::{ObjectStore, ObjectStoreScheme};
use s3s::dto;
use std::collections::HashMap;
use std::sync::Arc;
use url::Url;
pub mod in_memory;

#[derive(Debug, Clone)]
pub struct DataSource {
    name: String,
    region: String,
    url: String,
    creation_date: Option<dto::Timestamp>,
    credentials: HashMap<String, String>,
    http_connector: Option<Arc<dyn HttpConnector>>,
}

impl DataSource {
    pub fn as_object_store(self, prefix: Option<String>) -> Result<(Arc<dyn ObjectStore>, Path)> {
        let (object_store, root_prefix) = self.try_into()?;
        let mut full_path = format!("{}/{}", root_prefix, prefix.unwrap_or_default());
        if full_path.ends_with("/") {
            full_path = full_path.strip_suffix("/").unwrap().to_string();
        }
        Ok((object_store, Path::from(full_path)))
    }

    /// Create a new DataSource with the same configuration but with an HTTP connector
    pub fn with_http_connector<C>(self, http_connector: C) -> Self
    where
        C: HttpConnector,
    {
        Self {
            http_connector: Some(Arc::new(http_connector)),
            ..self
        }
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

impl TryFrom<DataSource> for (Arc<dyn ObjectStore>, Path) {
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

        // Parse the URL to determine the object store type
        let (scheme, path) =
            ObjectStoreScheme::parse(&url).map_err(|e| Error::ObjectStoreError(e.into()))?;

        // Build the object store using the appropriate builder with HTTP connector
        let object_store = match scheme {
            // #[cfg(feature = "aws")]
            ObjectStoreScheme::AmazonS3 => {
                let mut builder =
                    object_store::aws::AmazonS3Builder::new().with_url(url.to_string());

                // Apply all options
                for (key, value) in options {
                    if let Ok(config_key) = key.parse() {
                        builder = builder.with_config(config_key, value);
                    }
                    // Skip unknown keys silently
                }

                // Apply HTTP connector if provided
                if let Some(connector) = source.http_connector {
                    builder = builder.with_http_connector(connector.as_ref());
                }

                Box::new(builder.build()?) as Box<dyn ObjectStore>
            }
            _ => {
                return Err(Error::ObjectStoreError(object_store::Error::Generic {
                    store: "parse_url",
                    source: format!("feature for {scheme:?} not enabled").into(),
                }));
            }
        };

        Ok((Arc::new(object_store), path))
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
            http_connector: None,
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

    #[test]
    fn test_with_http_connector() {
        let source = create_test_source();
        assert!(source.http_connector.is_none());

        // Create a ReqwestConnector
        let connector = ReqwestConnector::default();
        let source_with_connector = source.with_http_connector(connector);
        assert!(source_with_connector.http_connector.is_some());

        // Verify other fields are preserved
        assert_eq!(source_with_connector.name, "test-bucket");
        assert_eq!(source_with_connector.region, "us-east-1");
        assert_eq!(source_with_connector.url, "s3://test-bucket");
    }
}
