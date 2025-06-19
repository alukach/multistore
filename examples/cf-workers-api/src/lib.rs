use bytes::Bytes;
use console_error_panic_hook;
use futures_util::TryStreamExt;
use http;
use http_body_util::BodyExt;
use http_body_util::Full;
use object_store::{
    aws::{AmazonS3Builder, AwsCredential},
    client::{
        ClientOptions, HttpClient, HttpConnector, HttpError, HttpErrorKind, HttpRequest,
        HttpResponse, HttpResponseBody, HttpService,
    },
    path::Path,
    ObjectStore, Result as ObjectStoreResult,
};
use worker::{
    async_trait, console_log, event, Context, Env, Fetch, Method, Request, Response,
    Result as CfResult,
};

#[event(fetch)]
async fn fetch(_req: Request, _env: Env, _ctx: Context) -> CfResult<Response> {
    console_error_panic_hook::set_once();

    let credentials = get_credentials();
    console_log!("credentials: {:?}", credentials);

    let client = HttpClient::new(FetchService);

    let store = AmazonS3Builder::new()
        .with_bucket_name("overturemaps-us-west-2")
        .with_region("us-west-2")
        .with_access_key_id(credentials.key_id)
        .with_secret_access_key(credentials.secret_key)
        .with_http_connector(FetchConnector::new(client))
        .build()
        .unwrap();

    let object = store.get(&Path::from("release/2025-05-21.0/theme=buildings/type=building/part-00006-0df994ca-3323-4d7c-a374-68c653f78289-c000.zstd.parquet")).await.unwrap();

    let mut headers = worker::Headers::new();
    headers.append("Content-Type", "application/octet-stream")?;
    headers.append("Transfer-Encoding", "chunked")?;

    let stream = object
        .into_stream()
        .map_err(|e| worker::Error::RustError(e.to_string()));

    Response::from_stream(stream).map(|resp| resp.with_headers(headers))
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

#[derive(Debug)]
struct FetchService;

impl FetchService {
    pub async fn fetch(&self, req: HttpRequest) -> Result<HttpResponse, HttpError> {
        use futures::channel::oneshot;
        use worker::wasm_bindgen_futures::spawn_local;

        let req = match Request::new(
            req.uri().to_string().as_str(),
            Method::from(req.method().to_string()),
        ) {
            Ok(req) => req,
            Err(e) => {
                return Err(HttpError::new(HttpErrorKind::Unknown, e));
            }
        };

        let (tx, rx) = oneshot::channel();

        spawn_local(async move {
            let fetch_request = Fetch::Request(req);
            let res_fut = fetch_request.send();
            let result = match res_fut.await {
                Ok(response) => {
                    let status = response.status_code();
                    let headers = response.headers();

                    // Convert headers to http::HeaderMap
                    let mut header_map = http::HeaderMap::new();
                    for (key, value) in headers.entries() {
                        if let Ok(name) = http::HeaderName::from_bytes(key.as_bytes()) {
                            if let Ok(value) = http::HeaderValue::from_str(&value) {
                                header_map.insert(name, value);
                            }
                        }
                    }

                    let (mut parts, _) = http::Response::new(()).into_parts();
                    parts.status = http::StatusCode::from_u16(status)
                        .unwrap_or(http::StatusCode::INTERNAL_SERVER_ERROR);
                    parts.headers = header_map;

                    // Create a dummy body - the real streaming will be handled by the extension
                    let dummy_body: Bytes = Bytes::from("Dummy Body");
                    let body = HttpResponseBody::new(Full::new(dummy_body).map_err(|e| match e {}));

                    Ok(HttpResponse::from_parts(parts, body))
                }
                Err(e) => Err(HttpError::new(HttpErrorKind::Unknown, e)),
            };

            let _ = tx.send(result);
        });

        rx.await.unwrap()
    }
}

#[async_trait::async_trait]
impl HttpService for FetchService {
    async fn call(&self, req: HttpRequest) -> Result<HttpResponse, HttpError> {
        self.fetch(req).await
    }
}
/// [`HttpConnector`] using [`Fetch`]
#[derive(Debug)]
pub struct FetchConnector {
    client: HttpClient,
}

impl FetchConnector {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }
}

impl HttpConnector for FetchConnector {
    fn connect(&self, _options: &ClientOptions) -> ObjectStoreResult<HttpClient> {
        Ok(self.client.clone())
    }
}
