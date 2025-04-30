use s3s::dto;

#[derive(Debug, Clone)]
pub struct VirtualBucket {
    name: String,
    region: String,
    alias: Option<String>,
}

/// A bucket registry is something that tracks the existence of buckets.
#[async_trait::async_trait]
pub trait BucketRegistry {
    async fn list_buckets(&self) -> Vec<dto::Bucket>;
}

pub struct YAMLDb {
    buckets: Vec<dto::Bucket>,
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
                let alias = v["alias"].as_str().unwrap().to_string();
                let region = v["region"].as_str().unwrap().to_string();
                dto::Bucket {
                    name: Some(alias),
                    creation_date: Some(dto::Timestamp::from(std::time::SystemTime::now())),
                    bucket_region: Some(region),
                }
            })
            .collect();
        Self { buckets }
    }
}

#[async_trait::async_trait]
impl BucketRegistry for YAMLDb {
    async fn list_buckets(&self) -> Vec<dto::Bucket> {
        self.buckets.clone()
    }
}
