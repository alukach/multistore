mod server;
mod utils;

use multistore::credentials::static_auth::StaticCredentialsRegistry;
use multistore::data_source::static_db::StaticDataSourceRegistry;
use multistore::error::Result;
use multistore::s3::S3Interface;
use s3s::service::S3ServiceBuilder;
use server::serve;
use utils::setup_tracing;

fn main() -> Result {
    setup_tracing();

    let creds_registry = StaticCredentialsRegistry::from_yaml("database.yaml");
    let data_source_registry = StaticDataSourceRegistry::from_yaml("database.yaml");
    let s3_backend = S3Interface::new(data_source_registry);

    let service = {
        let mut builder = S3ServiceBuilder::new(s3_backend);
        builder.set_auth(creds_registry);
        builder.build()
    };
    serve(service, "0.0.0.0:8080".parse().unwrap())
}
