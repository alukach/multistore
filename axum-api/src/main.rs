mod server;
mod utils;

use lib::credentials::yaml_auth::YAMLCredentialsRegistry;
use lib::data_source::yaml_db::InMemoryDataSourceRegistry;
use lib::error::Result;
use lib::s3::S3Interface;
use s3s::service::S3ServiceBuilder;
use server::serve;
use utils::setup_tracing;

fn main() -> Result {
    setup_tracing();

    let creds_registry = YAMLCredentialsRegistry::from_yaml("database.yaml");
    let data_source_registry = InMemoryDataSourceRegistry::from_yaml("database.yaml");
    let s3_backend = S3Interface::new(data_source_registry);

    let service = {
        let mut builder = S3ServiceBuilder::new(s3_backend);
        builder.set_auth(creds_registry);
        builder.build()
    };
    serve(service, "0.0.0.0:8080".parse().unwrap())
}
