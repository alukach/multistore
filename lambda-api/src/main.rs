mod utils;

use lambda_http::{service_fn, tracing, Body, Error, Request, Response};
use s3s::service::{S3Service, S3ServiceBuilder};
use std::env;
use std::sync::Arc;

use lib::credentials::static_auth::StaticCredentialsRegistry;
use lib::data_source::static_db::StaticDataSourceRegistry;
use lib::error::Result;
use lib::s3::S3Interface;
use utils::{convert_request, convert_response};

pub static S3_SERVICE: std::sync::OnceLock<Arc<S3Service>> = std::sync::OnceLock::new();

async fn handler(request: Request) -> Result<Response<Body>, Error> {
    let service = S3_SERVICE.get().expect("S3Service not initialized");
    let request = convert_request(request).await?;
    let response = service.call(request).await?;
    let response = convert_response(response).await?;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let service = {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        let db_path = current_dir.join("database.yaml");
        let db_path = db_path.to_str().unwrap();
        let creds_registry = StaticCredentialsRegistry::from_yaml(db_path);
        let data_source_registry = StaticDataSourceRegistry::from_yaml(db_path);
        let s3_backend = S3Interface::new(data_source_registry);

        let mut builder = S3ServiceBuilder::new(s3_backend);
        builder.set_auth(creds_registry);
        builder.build()
    };

    S3_SERVICE
        .set(Arc::new(service))
        .expect("S3 service already initialized");

    lambda_http::run(service_fn(handler)).await
}
