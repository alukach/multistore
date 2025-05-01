use crate::data_source::{DataSource, DataSourceRegistry};
use crate::error::Error;
use object_store::ObjectStore;
use s3s::dto;
use std::sync::Arc;

pub struct InMemoryDataSourceRegistry {
    data_sources: Vec<DataSource>,
}

impl InMemoryDataSourceRegistry {
    pub fn from_yaml(path: &str) -> Self {
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        let config: serde_yaml::Value = serde_yaml::from_reader(reader).unwrap();
        let data_sources = config["data-sources"].as_sequence().unwrap();
        let data_sources = data_sources
            .iter()
            .map(|v| {
                let name = v["name"].as_str().unwrap().to_string();
                let url = v["url"].as_str().or(Some(&name)).unwrap().to_string();
                let region = v["region"].as_str().unwrap().to_string();
                let creation_date = Some(dto::Timestamp::from(std::time::SystemTime::now()));

                DataSource {
                    name,
                    url,
                    region,
                    creation_date,
                }
            })
            .collect();
        Self { data_sources }
    }
}

#[async_trait::async_trait]
impl DataSourceRegistry for InMemoryDataSourceRegistry {
    async fn list_buckets(&self, _access_key: Option<&String>) -> Vec<dto::Bucket> {
        self.data_sources.iter().cloned().map(Into::into).collect()
    }

    async fn get_object_store(&self, bucket_name: &str) -> Result<Arc<dyn ObjectStore>, Error> {
        let bucket = self
            .data_sources
            .iter()
            .find(|b| b.name == bucket_name)
            .unwrap();
        Ok(bucket.try_into()?)
    }
}
