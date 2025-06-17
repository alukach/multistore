use console_error_panic_hook;
use object_store::{aws::AmazonS3Builder, path::Path};
use worker::{event, Context, Env, HttpRequest, Response, Result};

#[event(fetch)]
async fn fetch(req: HttpRequest, _env: Env, _ctx: Context) -> Result<Response> {
    // Initialize panic hook for better error messages
    console_error_panic_hook::set_once();

    let store = AmazonS3Builder::from_env()
        .with_bucket_name("overturemaps-us-west-2")
        .build()
        .map_err(|e| worker::Error::RustError(e.to_string()))?;

    let object = store
        .get(&Path::from("/release/2025-05-21.0/theme=buildings/type=building/part-00006-0df994ca-3323-4d7c-a374-68c653f78289-c000.zstd.parquet"))
        .await
        .map_err(|e| worker::Error::RustError(e.to_string()))?;

    let mut headers = worker::Headers::new();
    headers.append("Content-Type", "application/octet-stream")?;
    headers.append("Transfer-Encoding", "chunked")?;

    let stream = object
        .into_stream()
        .map_err(|e| worker::Error::RustError(e.to_string()));
    Response::from_stream(stream).map(|resp| resp.with_headers(headers))
}

// #[event(fetch)]
// async fn fetch(
//     req: HttpRequest,
//     _env: Env,
//     _ctx: Context,
// ) -> s3s::S3Result<http::Response<s3s::Body>> {
//     // Initialize panic hook for better error messages
//     console_error_panic_hook::set_once();

//     let config: serde_yaml::Value =
//         serde_yaml::from_str(include_str!("../../../database.yaml")).unwrap();

//     let creds_registry = StaticCredentialsRegistry::from_serde(config.clone());
//     let data_source_registry = StaticDataSourceRegistry::from_serde(config.clone());
//     let s3_backend = S3Interface::new(data_source_registry);

//     let service = {
//         let mut builder = S3ServiceBuilder::new(s3_backend);
//         builder.set_auth(creds_registry);
//         builder.build()
//     };

//     // Convert the request and handle it
//     let req = req.map(|body| s3s::Body::http_body(body));
//     service.call(req).await
// }

// struct ApiError(S3Error);

// impl From<ApiError> for worker::Error {
//     fn from(e: ApiError) -> Self {
//         worker::Error::RustError(e.0.to_string())
//     }
// }
