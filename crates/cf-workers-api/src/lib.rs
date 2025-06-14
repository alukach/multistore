use console_error_panic_hook;
use multistore::credentials::static_auth::StaticCredentialsRegistry;
use multistore::data_source::static_db::StaticDataSourceRegistry;
use multistore::s3::S3Interface;
use s3s::{service::S3ServiceBuilder, S3Error};
use worker::{event, Context, Env, HttpRequest};

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> s3s::S3Result<http::Response<s3s::Body>> {
    // Initialize panic hook for better error messages
    console_error_panic_hook::set_once();

    let config: serde_yaml::Value =
        serde_yaml::from_str(include_str!("../../../database.yaml")).unwrap();

    let creds_registry = StaticCredentialsRegistry::from_serde(config.clone());
    let data_source_registry = StaticDataSourceRegistry::from_serde(config.clone());
    let s3_backend = S3Interface::new(data_source_registry);

    let service = {
        let mut builder = S3ServiceBuilder::new(s3_backend);
        builder.set_auth(creds_registry);
        builder.build()
    };

    // Convert the request and handle it
    let req = req.map(|body| s3s::Body::http_body(body));
    service.call(req).await
}

struct ApiError(S3Error);

impl From<ApiError> for worker::Error {
    fn from(e: ApiError) -> Self {
        worker::Error::RustError(e.0.to_string())
    }
}
