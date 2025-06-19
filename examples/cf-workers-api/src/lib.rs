use bytes::Bytes;
use console_error_panic_hook;
use http;
use http_body_util::BodyExt;
use http_body_util::Full;
use hyper::Request as HyperRequest;
use multistore::credentials::static_auth::StaticCredentialsRegistry;
use multistore::data_source::static_db::StaticDataSourceRegistry;
use multistore::s3::S3Interface;
use object_store::{
    aws::AwsCredential,
    client::{
        ClientOptions, HttpClient, HttpConnector, HttpError, HttpErrorKind, HttpRequest,
        HttpResponse, HttpResponseBody, HttpService,
    },
    Result as ObjectStoreResult,
};
use s3s::service::S3ServiceBuilder;
use s3s::Body;
use web_sys::ReadableStream;
use worker::{
    async_trait, event, Context, Env, Fetch, Method, Request, Response, Result as CfResult,
};

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> CfResult<Response> {
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
    let res = service
        .call(WrappedRequest::from(req).into())
        .await
        .map_err(|e| worker::Error::RustError(e.to_string()))?;

    let stream: ReadableStream = if let Some(stream) = res.extensions().get::<WrappedStream>() {
        stream.stream.clone()
    } else {
        panic!("No stream found");
    };
    let response = Response::builder().stream(stream);
    Ok(response)
}

struct WrappedRequest(hyper::Request<s3s::Body>);

impl From<Request> for WrappedRequest {
    fn from(req: Request) -> Self {
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

// struct ApiError(S3Error);

// impl From<ApiError> for worker::Error {
//     fn from(e: ApiError) -> Self {
//         worker::Error::RustError(e.0.to_string())
//     }
// }

#[derive(Debug, Clone)]
struct WrappedStream {
    stream: ReadableStream,
}
unsafe impl Send for WrappedStream {}
unsafe impl Sync for WrappedStream {}

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
                    let headers: &worker::Headers = response.headers();
                    let stream = match response.body() {
                        worker::ResponseBody::Stream(body) => body,
                        _ => todo!(),
                    };

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
                    http_response.extensions_mut().insert(WrappedStream {
                        stream: stream.clone(),
                    });
                    Ok(http_response)
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
