use crate::data_source::{DataSource, DataSourceRegistry};
use crate::error::{Error, Result};
use s3s::dto;
use std::time::{SystemTime, UNIX_EPOCH};

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
                let creation_date = Some(dto::Timestamp::from(SystemTime::from(UNIX_EPOCH)));

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
}

#[async_trait::async_trait]
impl DataSourceRegistry for InMemoryDataSourceRegistry {
    async fn list_data_sources(
        &self,
        _access_key: Option<&String>,
        _input: dto::ListBucketsInput,
    ) -> Vec<DataSource> {
        self.data_sources.clone()
    }

    async fn get_data_source(&self, name: &str) -> Result<DataSource> {
        let Some(datasource) = self.data_sources.iter().find(|b| b.name == name) else {
            return Err(Error::DataSourceNotFound(name.to_string()));
        };

        Ok(datasource.clone())
    }
}
