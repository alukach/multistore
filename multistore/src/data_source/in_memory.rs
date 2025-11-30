use crate::data_source::{DataSource, DataSourcePage, DataSourceRegistry};
use crate::error::{Error, Result};
use object_store::client::HttpConnector;
use s3s::dto;
use std::time::UNIX_EPOCH;

#[derive(Clone)]
pub struct InMemoryDataSourceRegistry {
    data_sources: Vec<DataSource>,
}

impl InMemoryDataSourceRegistry {
    /// Create with inputted data sources
    pub fn from_data_sources(data_sources: Vec<DataSource>) -> Self {
        Self { data_sources }
    }

    /// Create with yaml file
    pub fn from_yaml(path: &str) -> Self {
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        let config: serde_yaml::Value = serde_yaml::from_reader(reader).unwrap();
        Self::from_serde(config)
    }

    pub fn from_serde(config: serde_yaml::Value) -> Self {
        let data_sources = config["data-sources"]
            .as_sequence()
            .unwrap()
            .iter()
            .map(|v| {
                let name = v["name"].as_str().unwrap().to_string();
                let url = v["url"].as_str().unwrap_or(&name).to_string();
                let region = v["region"].as_str().unwrap().to_string();
                let credentials = v["credentials"]
                    .as_mapping()
                    .unwrap()
                    .iter()
                    .map(|(k, v)| {
                        (
                            k.as_str().unwrap().to_string(),
                            v.as_str().unwrap().to_string(),
                        )
                    })
                    .collect();
                let creation_date = Some(dto::Timestamp::from(UNIX_EPOCH));

                DataSource {
                    name,
                    url,
                    region,
                    creation_date,
                    credentials,
                    http_connector: None,
                }
            })
            .collect();
        Self { data_sources }
    }

    /// Set an HTTP connector on all data sources in this registry
    pub fn with_http_connector<C>(mut self, http_connector: C) -> Self
    where
        C: HttpConnector + Clone,
    {
        self.data_sources = self
            .data_sources
            .into_iter()
            .map(|ds| ds.with_http_connector(http_connector.clone()))
            .collect();
        self
    }
}

#[async_trait::async_trait]
impl DataSourceRegistry for InMemoryDataSourceRegistry {
    async fn list_data_sources(
        &self,
        _access_key: Option<&String>,
        input: dto::ListBucketsInput,
    ) -> DataSourcePage {
        let mut sources = self.data_sources.clone();

        // Apply prefix filter if specified
        if let Some(ref prefix) = input.prefix {
            sources.retain(|s| s.name.starts_with(prefix.as_str()));
        }

        // Sort for consistent ordering (required for stable pagination)
        sources.sort_by(|a, b| a.name.cmp(&b.name));

        // Handle continuation token to find start position
        let start_idx = if let Some(ref token) = input.continuation_token {
            // Use the token as a marker - find the first bucket name that comes after it
            sources
                .iter()
                .position(|s| s.name.as_str() > token.as_str())
                .unwrap_or(sources.len())
        } else {
            0
        };

        // Apply max_buckets limit (default: 10000 per S3 spec when pagination params are used)
        let max = input.max_buckets.unwrap_or(10000) as usize;
        let end_idx = (start_idx + max).min(sources.len());

        let page_sources = sources[start_idx..end_idx].to_vec();

        // Generate continuation token if more results exist
        // Use the last bucket name in this page as the token
        let continuation_token = if end_idx < sources.len() {
            page_sources.last().map(|s| s.name.clone())
        } else {
            None
        };

        DataSourcePage {
            data_sources: page_sources,
            continuation_token,
        }
    }

    async fn get_data_source(&self, name: &str) -> Result<DataSource> {
        let Some(datasource) = self.data_sources.iter().find(|b| b.name == name) else {
            return Err(Error::DataSourceNotFound(name.to_string()));
        };

        Ok(datasource.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_data_source(name: &str) -> DataSource {
        DataSource {
            name: name.to_string(),
            region: "us-east-1".to_string(),
            url: format!("s3://{}", name),
            creation_date: None,
            credentials: HashMap::new(),
            http_connector: None,
        }
    }

    #[tokio::test]
    async fn test_list_all_buckets_no_pagination() {
        let registry = InMemoryDataSourceRegistry::from_data_sources(vec![
            create_test_data_source("bucket-a"),
            create_test_data_source("bucket-b"),
            create_test_data_source("bucket-c"),
        ]);

        let input = dto::ListBucketsInput::default();
        let page = registry.list_data_sources(None, input).await;

        assert_eq!(page.data_sources.len(), 3);
        assert!(page.continuation_token.is_none());
        assert_eq!(page.data_sources[0].name, "bucket-a");
        assert_eq!(page.data_sources[1].name, "bucket-b");
        assert_eq!(page.data_sources[2].name, "bucket-c");
    }

    #[tokio::test]
    async fn test_list_buckets_with_max_buckets() {
        let registry = InMemoryDataSourceRegistry::from_data_sources(vec![
            create_test_data_source("bucket-a"),
            create_test_data_source("bucket-b"),
            create_test_data_source("bucket-c"),
            create_test_data_source("bucket-d"),
            create_test_data_source("bucket-e"),
        ]);

        // Request first page with max 2 buckets
        let input = dto::ListBucketsInput {
            max_buckets: Some(2),
            ..Default::default()
        };
        let page = registry.list_data_sources(None, input).await;

        assert_eq!(page.data_sources.len(), 2);
        assert_eq!(page.data_sources[0].name, "bucket-a");
        assert_eq!(page.data_sources[1].name, "bucket-b");
        assert!(page.continuation_token.is_some());
        assert_eq!(page.continuation_token.as_ref().unwrap(), "bucket-b");
    }

    #[tokio::test]
    async fn test_list_buckets_pagination_flow() {
        let registry = InMemoryDataSourceRegistry::from_data_sources(vec![
            create_test_data_source("bucket-a"),
            create_test_data_source("bucket-b"),
            create_test_data_source("bucket-c"),
            create_test_data_source("bucket-d"),
            create_test_data_source("bucket-e"),
        ]);

        // Page 1: First 2 buckets
        let input1 = dto::ListBucketsInput {
            max_buckets: Some(2),
            ..Default::default()
        };
        let page1 = registry.list_data_sources(None, input1).await;
        assert_eq!(page1.data_sources.len(), 2);
        assert_eq!(page1.data_sources[0].name, "bucket-a");
        assert_eq!(page1.data_sources[1].name, "bucket-b");
        assert_eq!(page1.continuation_token.as_ref().unwrap(), "bucket-b");

        // Page 2: Next 2 buckets using continuation token
        let input2 = dto::ListBucketsInput {
            max_buckets: Some(2),
            continuation_token: page1.continuation_token.clone(),
            ..Default::default()
        };
        let page2 = registry.list_data_sources(None, input2).await;
        assert_eq!(page2.data_sources.len(), 2);
        assert_eq!(page2.data_sources[0].name, "bucket-c");
        assert_eq!(page2.data_sources[1].name, "bucket-d");
        assert_eq!(page2.continuation_token.as_ref().unwrap(), "bucket-d");

        // Page 3: Last bucket
        let input3 = dto::ListBucketsInput {
            max_buckets: Some(2),
            continuation_token: page2.continuation_token.clone(),
            ..Default::default()
        };
        let page3 = registry.list_data_sources(None, input3).await;
        assert_eq!(page3.data_sources.len(), 1);
        assert_eq!(page3.data_sources[0].name, "bucket-e");
        assert!(page3.continuation_token.is_none()); // No more pages
    }

    #[tokio::test]
    async fn test_list_buckets_with_prefix() {
        let registry = InMemoryDataSourceRegistry::from_data_sources(vec![
            create_test_data_source("prod-bucket-a"),
            create_test_data_source("prod-bucket-b"),
            create_test_data_source("test-bucket-a"),
            create_test_data_source("test-bucket-b"),
        ]);

        let input = dto::ListBucketsInput {
            prefix: Some("prod-".to_string()),
            ..Default::default()
        };
        let page = registry.list_data_sources(None, input).await;

        assert_eq!(page.data_sources.len(), 2);
        assert_eq!(page.data_sources[0].name, "prod-bucket-a");
        assert_eq!(page.data_sources[1].name, "prod-bucket-b");
        assert!(page.continuation_token.is_none());
    }

    #[tokio::test]
    async fn test_list_buckets_with_prefix_and_pagination() {
        let registry = InMemoryDataSourceRegistry::from_data_sources(vec![
            create_test_data_source("prod-bucket-a"),
            create_test_data_source("prod-bucket-b"),
            create_test_data_source("prod-bucket-c"),
            create_test_data_source("test-bucket-a"),
        ]);

        let input = dto::ListBucketsInput {
            prefix: Some("prod-".to_string()),
            max_buckets: Some(2),
            ..Default::default()
        };
        let page = registry.list_data_sources(None, input).await;

        assert_eq!(page.data_sources.len(), 2);
        assert_eq!(page.data_sources[0].name, "prod-bucket-a");
        assert_eq!(page.data_sources[1].name, "prod-bucket-b");
        assert!(page.continuation_token.is_some());

        // Get next page
        let input2 = dto::ListBucketsInput {
            prefix: Some("prod-".to_string()),
            max_buckets: Some(2),
            continuation_token: page.continuation_token,
            ..Default::default()
        };
        let page2 = registry.list_data_sources(None, input2).await;
        assert_eq!(page2.data_sources.len(), 1);
        assert_eq!(page2.data_sources[0].name, "prod-bucket-c");
        assert!(page2.continuation_token.is_none());
    }

    #[tokio::test]
    async fn test_list_buckets_empty_result() {
        let registry = InMemoryDataSourceRegistry::from_data_sources(vec![]);

        let input = dto::ListBucketsInput::default();
        let page = registry.list_data_sources(None, input).await;

        assert_eq!(page.data_sources.len(), 0);
        assert!(page.continuation_token.is_none());
    }

    #[tokio::test]
    async fn test_list_buckets_stable_ordering() {
        // Create buckets in non-alphabetical order
        let registry = InMemoryDataSourceRegistry::from_data_sources(vec![
            create_test_data_source("bucket-z"),
            create_test_data_source("bucket-a"),
            create_test_data_source("bucket-m"),
        ]);

        let input = dto::ListBucketsInput::default();
        let page = registry.list_data_sources(None, input).await;

        // Should be sorted alphabetically
        assert_eq!(page.data_sources[0].name, "bucket-a");
        assert_eq!(page.data_sources[1].name, "bucket-m");
        assert_eq!(page.data_sources[2].name, "bucket-z");
    }
}
