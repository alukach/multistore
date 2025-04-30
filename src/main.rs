mod auth;
mod data_source;
mod error;
mod s3;
mod server;
mod utils;

use auth::yaml_auth::YAMLAuth;
use data_source::yaml_db::YAMLDb;
use error::Result;
use s3::S3Interface;
use s3s::service::S3ServiceBuilder;
use server::serve;
use std::sync::Arc;
use utils::setup_tracing;

fn main() -> Result {
    setup_tracing();

    let s3_backend = S3Interface::new(Arc::new(YAMLDb::from_yaml("database.yaml")));
    let auth = YAMLAuth::from_yaml("database.yaml");

    let service = {
        let mut builder = S3ServiceBuilder::new(s3_backend);
        builder.set_auth(auth);

        // // Enable parsing virtual-hosted-style requests
        // if opt.domain.is_empty().not() {
        //     builder.set_host(MultiDomain::new(&opt.domain)?);
        //     info!("virtual-hosted-style requests are enabled");
        // }

        builder.build()
    };
    serve(service, "127.0.0.1:8080".parse().unwrap())
}
