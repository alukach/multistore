/// A data source registery is the tooling to track the existence of data sources (ie 
/// various Object Storage backends).
use s3s::dto;
pub mod yaml_db;


#[derive(Debug, Clone)]
pub struct DataSource {
    name: String,
    region: String,
    alias: String,
    creation_date: Option<dto::Timestamp>,
}

/// A bucket registry 
#[async_trait::async_trait]
pub trait DataSourceRegistry {
    async fn list_buckets(&self, access_key: Option<&String>) -> Vec<dto::Bucket>;
}
