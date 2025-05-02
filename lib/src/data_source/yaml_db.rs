use crate::data_source::{DataSource, DataSourceRegistry};
use crate::error::Error;
use object_store::{ObjectStore, path::Path};
use s3s::dto;

#[derive(Clone)]
pub struct InMemoryDataSourceRegistry {
    data_sources: Vec<DataSource>,
}

impl InMemoryDataSourceRegistry {
    pub fn from_yaml(path: &str) -> Self {
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        let config: serde_yaml::Value = serde_yaml::from_reader(reader).unwrap();
        let data_sources = config["data-sources"]
            .as_sequence()
            .unwrap()
            .iter()
            .map(|v| {
                let name = v["name"].as_str().unwrap().to_string();
                let url = v["url"].as_str().or(Some(&name)).unwrap().to_string();
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
                let creation_date = Some(dto::Timestamp::from(std::time::SystemTime::now()));

                DataSource {
                    name,
                    url,
                    region,
                    creation_date,
                    credentials,
                }
            })
            .collect();
        Self { data_sources }
    }
}

#[async_trait::async_trait]
impl DataSourceRegistry for InMemoryDataSourceRegistry {
    async fn list_data_sources(&self, _access_key: Option<&String>) -> Vec<DataSource> {
        self.data_sources.clone()
    }

    async fn get_object_store(
        &self,
        bucket_name: &str,
    ) -> Result<(Box<dyn ObjectStore>, Path), Error> {
        let Some(datasource) = self.data_sources.iter().find(|b| b.name == bucket_name) else {
            return Err(Error::from_string("Bucket not found"));
        };

        Ok(datasource.clone().try_into()?)
    }
}
