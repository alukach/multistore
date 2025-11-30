mod wasm;

use crate::wasm::{take_global_stream, FetchConnector};
use console_error_panic_hook;
use multistore::credentials::in_memory::InMemoryCredentialsRegistry;
use multistore::data_source::in_memory::InMemoryDataSourceRegistry;
use multistore::s3::S3Interface;
use s3s::{service::S3ServiceBuilder, S3Error, S3ErrorCode};

#[worker::event(fetch)]
async fn fetch(
    req: worker::HttpRequest,
    _env: worker::Env,
    _ctx: worker::Context,
) -> s3s::S3Result<worker::Response> {
    // Initialize panic hook for better error messages
    console_error_panic_hook::set_once();

    let config: serde_yaml::Value =
        serde_yaml::from_str(include_str!("../../../database.yaml")).unwrap();

    let creds_registry = InMemoryCredentialsRegistry::from_serde(config.clone());

    // Create data source registry with HTTP connector support
    let connector = FetchConnector {};
    let data_source_registry =
        InMemoryDataSourceRegistry::from_serde(config.clone()).with_http_connector(connector);

    let s3_backend = S3Interface::new(data_source_registry);

    let service = {
        let mut builder = S3ServiceBuilder::new(s3_backend);
        builder.set_auth(creds_registry);
        builder.build()
    };

    // Convert the request and handle it
    let req = req.map(|body| s3s::Body::http_body(body));
    let res = match service.call(req).await {
        Ok(res) => res,
        Err(e) => {
            worker::console_error!("Received error from S3S service: {:?}", e);
            return Err(S3Error::new(S3ErrorCode::InternalError));
        }
    };

    // Build response with ReadableStream
    let headers = {
        let headers = worker::Headers::new();
        for (key, value) in res.headers().iter() {
            let _ = headers.append(key.as_str(), value.to_str().unwrap());
        }
        headers
    };
    let res_with_body = {
        let response = worker::ResponseBuilder::new();
        if let Some(stream) = take_global_stream() {
            worker::console_debug!("Stream available, using it as body");
            response.stream(stream)
        } else {
            worker::console_debug!("Stream not available, using s3s response body");
            let body = res.body().bytes().unwrap_or_default();
            response.fixed(body.into())
        }
    };
    worker::console_log!("Responding with status: {:?}", res.status().as_u16());
    worker::console_log!("Responding with headers: {:?}", headers);
    Ok(res_with_body
        .with_headers(headers)
        .with_status(res.status().as_u16()))
}
