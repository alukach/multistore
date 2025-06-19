use bytes::Bytes;
use console_error_panic_hook;
use http;
use http_body_util::BodyExt;
use http_body_util::Full;
use hyper::Request as HyperRequest;
use object_store::{
    aws::{AmazonS3Builder, AwsCredential},
    client::{
        ClientOptions, HttpClient, HttpConnector, HttpError, HttpErrorKind, HttpRequest,
        HttpResponse, HttpResponseBody, HttpService,
    },
    path::Path,
    ObjectStore, Result as ObjectStoreResult,
};
use std::pin::Pin;
use std::task::{Context, Poll};
use web_sys::ReadableStream;

// Global storage for the ReadableStream - safe in single-threaded WASM
static mut GLOBAL_STREAM: Option<ReadableStream> = None;

// Helper functions to safely access the global stream
fn set_global_stream(stream: ReadableStream) {
    unsafe {
        GLOBAL_STREAM = Some(stream);
    }
}

fn take_global_stream() -> Option<ReadableStream> {
    unsafe { GLOBAL_STREAM.take() }
}

// Wrapper to convert ReadableStream to a futures Stream
struct ReadableStreamWrapper {
    stream: ReadableStream,
}

impl ReadableStreamWrapper {
    fn new(stream: ReadableStream) -> Self {
        Self { stream }
    }
}

impl futures::Stream for ReadableStreamWrapper {
    type Item = Result<Bytes, worker::Error>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // For now, return a simple implementation
        // In a real implementation, you would need to properly read from the ReadableStream
        // This is a placeholder that returns a single chunk
        Poll::Ready(Some(Ok(Bytes::from("data from stream"))))
    }
}

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

    let client = HttpClient::new(FetchService);

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

struct WrappedRequest(hyper::Request<s3s::Body>);

impl From<worker::Request> for WrappedRequest {
    fn from(req: worker::Request) -> Self {
        let mut hyper_req = hyper::Request::new(s3s::Body::empty());
        *hyper_req.method_mut() =
            hyper::Method::from_bytes(req.method().to_string().as_bytes()).unwrap();
        // *hyper_req.uri_mut() = hyper::Uri::from_static(&req.url().unwrap().to_string());
        // *hyper_req.headers_mut() =
        //     hyper::HeaderMap::from_iter(req.headers().entries().map(|(key, value)| {
        //         (
        //             hyper::Head::from_bytes(key.as_bytes()).unwrap(),
        //             hyper::HeaderValue::from_str(value).unwrap(),
        //         )
        //     }));
        // TODO: add body
        Self(hyper_req)
    }
}

impl From<WrappedRequest> for HyperRequest<s3s::Body> {
    fn from(req: WrappedRequest) -> Self {
        req.0
    }
}

#[derive(Debug)]
struct FetchService;

impl FetchService {
    pub async fn fetch(&self, req: HttpRequest) -> Result<HttpResponse, HttpError> {
        use futures::channel::oneshot;
        use worker::wasm_bindgen_futures::spawn_local;

        let req = match worker::Request::new(
            req.uri().to_string().as_str(),
            worker::Method::from(req.method().to_string()),
        ) {
            Ok(req) => req,
            Err(e) => {
                return Err(HttpError::new(HttpErrorKind::Unknown, e));
            }
        };

        let (tx, rx) = oneshot::channel();

        spawn_local(async move {
            let fetch_request = worker::Fetch::Request(req);
            let res_fut = fetch_request.send();
            let result = match res_fut.await {
                Ok(response) => {
                    let status = response.status_code();
                    let headers: &worker::Headers = response.headers();
                    let stream = match response.body() {
                        worker::ResponseBody::Stream(body) => body,
                        _ => todo!(),
                    };

                    // Store the stream globally so we can access it later
                    set_global_stream(stream.clone());

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

                    let mut http_response = HttpResponse::from_parts(parts, body);
                    Ok(http_response)
                }
                Err(e) => Err(HttpError::new(HttpErrorKind::Unknown, e)),
            };

            let _ = tx.send(result);
        });

        rx.await.unwrap()
    }
}

#[worker::async_trait::async_trait]
impl HttpService for FetchService {
    async fn call(&self, req: HttpRequest) -> Result<HttpResponse, HttpError> {
        self.fetch(req).await
    }
}

#[derive(Debug, Default)]
struct FetchConnector {}

impl HttpConnector for FetchConnector {
    fn connect(&self, _options: &ClientOptions) -> ObjectStoreResult<HttpClient> {
        let client = FetchService {};
        Ok(HttpClient::new(client))
    }
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
