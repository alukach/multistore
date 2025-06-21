use bytes::Bytes;
use futures::StreamExt;
use object_store::{
    client::{
        HttpClient, HttpConnector, HttpError, HttpErrorKind, HttpRequest, HttpResponseBody,
        HttpService,
    },
    ClientOptions,
};
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

        worker::console_debug!("Fetching request");

        let (tx, rx) = oneshot::channel();

        spawn_local(async move {
            let mut res = match worker::Fetch::Request(req).send().await {
                Ok(res) => res,
                Err(e) => {
                    let _ = tx.send(Err(HttpError::new(HttpErrorKind::Unknown, e)));
                    return;
                }
            };

            // NOTE: We need to clone the response to allow us to send a stream to both
            // the global and to the handler. It seems that even calling `request.body()`
            // disturbs the stream, so we need to clone it before that.
            let mut res_dup = res.cloned().unwrap();

            let body = match res.body() {
                worker::ResponseBody::Stream(body) => {
                    worker::console_debug!("Found stream body, setting global stream");
                    set_global_stream(body.clone());
                    byte_stream_to_http_body(res_dup.stream().unwrap()).await
                }
                worker::ResponseBody::Body(body) => {
                    worker::console_debug!("Found non-stream body, returning body");
                    Bytes::from(body.to_vec()).into()
                }
                worker::ResponseBody::Empty => {
                    worker::console_debug!("Found empty body, returning empty body");
                    Bytes::new().into()
                }
            };

            let mut http_response = object_store::client::HttpResponse::new(body);
            *http_response.status_mut() = http::StatusCode::from_u16(res.status_code()).unwrap();
            *http_response.headers_mut() = res.headers().into();
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
    worker::console_debug!("Setting global stream");
    unsafe {
        GLOBAL_STREAM = Some(stream);
    }
}

pub fn take_global_stream() -> Option<ReadableStream> {
    worker::console_debug!("Taking global stream");
    unsafe { GLOBAL_STREAM.take() }
}

/// Helper to convert your ByteStream → HttpResponseBody
async fn byte_stream_to_http_body(mut stream: worker::ByteStream) -> HttpResponseBody {
    use futures::channel::mpsc;
    use futures::SinkExt;
    use http_body_util::StreamBody;
    use worker::wasm_bindgen_futures::spawn_local;

    let (mut tx, rx) = mpsc::channel(1);

    // Spawn a task to read from the ByteStream and send to the channel
    spawn_local(async move {
        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(bytes) => {
                    if let Err(_) = tx.send(Ok(bytes)).await {
                        // Receiver was dropped, stop processing
                        break;
                    }
                }
                Err(e) => {
                    let _ = tx
                        .send(Err(HttpError::new(HttpErrorKind::Unknown, e)))
                        .await;
                    break;
                }
            }
        }
        unsafe {
            worker::console_debug!("Stream was read, clearing global stream");
            GLOBAL_STREAM = None;
        }
    });

    // Create a stream that maps the channel receiver to Frame::data
    let safe_stream = rx.map(|chunk| {
        let frame = hyper::body::Frame::data(Bytes::from(chunk?));
        Ok(frame)
    });

    HttpResponseBody::new(StreamBody::new(safe_stream))
}
