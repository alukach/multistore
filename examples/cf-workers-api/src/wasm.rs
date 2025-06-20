use bytes::Bytes;
use http;
use http_body_util::{BodyExt, Full};
use object_store::{
    client::{
        HttpClient, HttpConnector, HttpError, HttpErrorKind, HttpRequest, HttpResponseBody,
        HttpService,
    },
    ClientOptions,
};
use std::io;
use web_sys::ReadableStream;
use worker;

#[derive(Debug)]
pub struct FetchService;

impl FetchService {
    pub async fn fetch(
        &self,
        req: worker::Request,
    ) -> Result<object_store::client::HttpResponse, HttpError> {
        use futures::channel::oneshot;
        use worker::wasm_bindgen_futures::spawn_local;

        let (tx, rx) = oneshot::channel();

        spawn_local(async move {
            let fetch_request = worker::Fetch::Request(req);
            let result = fetch_request.send().await;

            let response = match result {
                Ok(response) => response,
                Err(e) => {
                    let _ = tx.send(Err(HttpError::new(HttpErrorKind::Unknown, e)));
                    return;
                }
            };

            if let worker::ResponseBody::Stream(body) = response.body() {
                // Store the stream globally so we can access it later
                set_global_stream(body.clone());
            } else {
                let _ = tx.send(Err(HttpError::new(
                    HttpErrorKind::Unknown,
                    io::Error::new(io::ErrorKind::Other, "Expected stream body"),
                )));
                return;
            }

            let (mut parts, _) = http::Response::new(()).into_parts();
            parts.status = http::StatusCode::from_u16(response.status_code())
                .unwrap_or(http::StatusCode::INTERNAL_SERVER_ERROR);
            parts.headers = response
                .headers()
                .entries()
                .filter_map(|(key, value)| {
                    let name = http::HeaderName::from_bytes(key.as_bytes()).ok()?;
                    let value = http::HeaderValue::from_str(&value).ok()?;
                    Some((name, value))
                })
                .collect();

            // Create a dummy body - the real stream body will be retrieved from the global stream
            let body =
                HttpResponseBody::new(Full::new(Bytes::from("Dummy Body")).map_err(|e| match e {}));

            let http_response = object_store::client::HttpResponse::from_parts(parts, body);
            let _ = tx.send(Ok(http_response));
        });

        rx.await.unwrap()
    }
}

#[worker::async_trait::async_trait]
impl HttpService for FetchService {
    async fn call(
        &self,
        req: HttpRequest,
    ) -> Result<object_store::client::HttpResponse, HttpError> {
        let req = match worker::Request::new(
            req.uri().to_string().as_str(),
            worker::Method::from(req.method().to_string()),
        ) {
            Ok(req) => req,
            Err(e) => {
                return Err(HttpError::new(HttpErrorKind::Unknown, e));
            }
        };
        self.fetch(req).await
    }
}

#[derive(Debug, Default, Clone)]
pub struct FetchConnector {}

impl HttpConnector for FetchConnector {
    fn connect(&self, _options: &ClientOptions) -> object_store::Result<HttpClient> {
        let client = FetchService {};
        Ok(HttpClient::new(client))
    }
}

// Global storage for the ReadableStream - safe in single-threaded WASM
static mut GLOBAL_STREAM: Option<ReadableStream> = None;

// Helper functions to safely access the global stream
fn set_global_stream(stream: ReadableStream) {
    unsafe {
        GLOBAL_STREAM = Some(stream);
    }
}

pub fn take_global_stream() -> Option<ReadableStream> {
    unsafe { GLOBAL_STREAM.take() }
}
