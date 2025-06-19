mod fetch;
mod stream;

use console_error_panic_hook;
use object_store::{
    aws::{AmazonS3Builder, AwsCredential},
    path::Path,
    ObjectStore,
};

use fetch::FetchConnector;
use stream::take_global_stream;

#[worker::event(fetch)]
async fn fetch(
    _req: worker::Request,
    _env: worker::Env,
    _ctx: worker::Context,
) -> worker::Result<worker::Response> {
    // Initialize panic hook for better error messages
    console_error_panic_hook::set_once();

    let credentials = get_credentials();
    worker::console_log!("credentials: {:?}", credentials);

    let store = AmazonS3Builder::new()
        .with_bucket_name("overturemaps-us-west-2")
        .with_region("us-west-2")
        .with_access_key_id(credentials.key_id)
        .with_secret_access_key(credentials.secret_key)
        .with_http_connector(FetchConnector {})
        .build()
        .unwrap();

    let object = store
        .get(&Path::from("/release/2025-05-21.0/theme=buildings/type=building/part-00006-0df994ca-3323-4d7c-a374-68c653f78289-c000.zstd.parquet"))
        .await
        .map_err(|e| worker::Error::RustError(e.to_string()))?;

    let mut headers = worker::Headers::new();
    headers.append("Content-Type", "application/octet-stream")?;
    headers.append("Transfer-Encoding", "chunked")?;

    // Retrieve the stored ReadableStream from the global variable
    let stream = take_global_stream()
        .ok_or_else(|| worker::Error::RustError("No stream available".to_string()))?;

    Ok(worker::ResponseBuilder::new()
        .stream(stream)
        .with_headers(headers))
}

fn get_credentials() -> AwsCredential {
    let config: serde_yaml::Value =
        serde_yaml::from_str(include_str!("../../../database.yaml")).unwrap();
    let data_sources = config["data-sources"].as_sequence().unwrap();
    AwsCredential {
        key_id: data_sources[0]["credentials"]["access_key_id"]
            .as_str()
            .unwrap()
            .to_string(),
        secret_key: data_sources[0]["credentials"]["secret_access_key"]
            .as_str()
            .unwrap()
            .to_string(),
        token: None,
    }
}
