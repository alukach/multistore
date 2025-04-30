use crate::data_source::{DataSourceRegistry, DataSource};
use s3s::dto;

pub struct YAMLDb {
    buckets: Vec<DataSource>,
}

impl YAMLDb {
    pub fn from_yaml(path: &str) -> Self {
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        let config: serde_yaml::Value = serde_yaml::from_reader(reader).unwrap();
        let buckets = config["buckets"].as_sequence().unwrap();
        let buckets = buckets
            .iter()
            .map(|v| {
                let name = v["name"].as_str().unwrap().to_string();
                let alias = v["alias"].as_str().or(Some(&name)).unwrap().to_string();
                let region = v["region"].as_str().unwrap().to_string();
                let creation_date = Some(dto::Timestamp::from(std::time::SystemTime::now()));

                DataSource {
                    name: name,
                    alias: alias,
                    region: region,
                    creation_date,
                }
            })
            .collect();
        Self { buckets }
    }
}

#[async_trait::async_trait]
impl DataSourceRegistry for YAMLDb {
    async fn list_buckets(&self, _access_key: Option<&String>) -> Vec<dto::Bucket> {
        self.buckets
            .iter()
            .map(|b| dto::Bucket {
                name: Some(b.alias.clone()),
                creation_date: b.creation_date.clone(),
                bucket_region: Some(b.region.clone()),
            })
            .collect()
    }
}
