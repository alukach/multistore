use bytes::Bytes;
use futures::SinkExt;
use futures::StreamExt;
use futures::channel::mpsc;
use http_body_util::StreamBody;
use hyper::body::Frame;
use object_store::{
    ClientOptions,
    client::{
        HttpClient, HttpConnector, HttpError, HttpErrorKind, HttpRequest, HttpResponseBody,
        HttpService,
    },
};
use std::cell::RefCell;
use web_sys::ReadableStream;
use worker;
use worker::wasm_bindgen_futures::spawn_local;
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
            let mut res = match worker::Fetch::Request(req).send().await {
                Ok(res) => res,
                Err(e) => {
                    let _ = tx.send(Err(HttpError::new(HttpErrorKind::Unknown, e)));
                    return;
                }
            };

            // NOTE: We need to clone the response to allow us to send a stream to both
            // the global stream store and to the handler. It seems that even calling
            // `res.body()` disturbs the stream, so we need to clone it before that.
            let mut res_dup = res.cloned().unwrap();

            let body = match res.body() {
                worker::ResponseBody::Stream(body) => {
                    worker::console_debug!("Found stream body, setting global stream");
                    set_global_stream(body.clone());
                    let bytestream = res_dup.stream().unwrap();
                    // Streams are used in both `ls` and `cp` commands
                    byte_stream_to_http_body(bytestream).await
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
        req_in: HttpRequest,
    ) -> Result<object_store::client::HttpResponse, HttpError> {
        let req_out = match worker::Request::new(
            req_in.uri().to_string().as_str(),
            worker::Method::from(req_in.method().to_string()),
        ) {
            Ok(mut req_out) => {
                req_in.headers().iter().for_each(|(key, value)| {
                    req_out
                        .headers_mut()
                        .expect("Failed to get headers")
                        .set(key.as_str(), value.to_str().unwrap())
                        .expect("Failed to set header");
                });
                req_out
            }
            Err(e) => {
                return Err(HttpError::new(HttpErrorKind::Unknown, e));
            }
        };
        self.fetch(req_out).await
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

// a thread-local global, initialized to None
thread_local! {
    static GLOBAL_STREAM: RefCell<Option<ReadableStream>> = RefCell::new(None);
}

/// Store it
pub fn set_global_stream(stream: ReadableStream) {
    worker::console_debug!("Setting global stream");
    GLOBAL_STREAM.with(|cell| {
        // replace the old value, dropping if any
        cell.replace(Some(stream));
    });
}

/// Take it out (leaving None behind)
pub fn take_global_stream() -> Option<ReadableStream> {
    worker::console_debug!("Taking global stream");
    GLOBAL_STREAM.with(|cell| {
        // take ownership of the Option, leaving None
        cell.replace(None)
    })
}
/// Helper to convert your ByteStream → HttpResponseBody
async fn byte_stream_to_http_body(mut stream: worker::ByteStream) -> HttpResponseBody {
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
    });

    // Create a stream that maps the channel receiver to Frame::data
    let safe_stream = rx.map(|chunk| {
        let frame = Frame::data(Bytes::from(chunk?));
        Ok(frame)
    });

    HttpResponseBody::new(StreamBody::new(safe_stream))
}
