use lib::credentials::yaml_auth::YAMLCredentialsRegistry;
use lib::data_source::yaml_db::InMemoryDataSourceRegistry;
use lib::s3::S3Interface;
use s3s::{service::S3ServiceBuilder, S3Error};
use worker::{event, Context, Env, HttpRequest};

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> s3s::S3Result<http::Response<s3s::Body>> {
    let creds_registry = YAMLCredentialsRegistry::from_yaml("database.yaml");
    let data_source_registry = InMemoryDataSourceRegistry::from_yaml("database.yaml");
    let s3_backend = S3Interface::new(data_source_registry);

    let service = {
        let mut builder = S3ServiceBuilder::new(s3_backend);
        builder.set_auth(creds_registry);
        builder.build()
    };

    service
        .call(req.map(|body| s3s::Body::http_body(body)))
        .await
}

struct ApiError(S3Error);

impl From<ApiError> for worker::Error {
    fn from(e: ApiError) -> Self {
        worker::Error::RustError(e.0.to_string())
    }
}
