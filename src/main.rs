mod error;
mod s3;
mod server;
mod utils;

use error::Result;
use s3::ObjectStore;
use s3s::auth::SimpleAuth;
use s3s::service::{S3Service, S3ServiceBuilder};
use server::serve;
use utils::setup_tracing;

fn build_service() -> S3Service {
    let s3_backend = ObjectStore {};
    let mut builder = S3ServiceBuilder::new(s3_backend);
    builder.set_auth(SimpleAuth::from_single("foo", "bar"));

    // // Enable parsing virtual-hosted-style requests
    // if opt.domain.is_empty().not() {
    //     builder.set_host(MultiDomain::new(&opt.domain)?);
    //     info!("virtual-hosted-style requests are enabled");
    // }

    builder.build()
}

fn main() -> Result {
    setup_tracing();
    let service = build_service();
    serve(service, "127.0.0.1:8080".parse().unwrap())
}
