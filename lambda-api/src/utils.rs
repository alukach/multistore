use http_body_util::BodyExt;
use lambda_http::Error;
use lambda_http::{Body, Request, Response};
use lib::error::Result;

pub async fn convert_request(event: Request) -> Result<hyper::Request<s3s::Body>, Error> {
    // Clone necessary parts before moving
    let method = event.method().clone();
    let uri = event.uri().clone();
    let headers = event.headers().clone();

    // Convert lambda_http::Body to hyper::body::Bytes
    let body = event.into_body();
    let bytes = body.collect().await?.to_bytes();

    // Create a new hyper::Request with the converted body
    let mut event = hyper::Request::new(bytes.into());
    *event.method_mut() = method;
    *event.uri_mut() = uri;
    *event.headers_mut() = headers;

    Ok(event)
}

pub async fn convert_response(
    response: hyper::Response<s3s::Body>,
) -> Result<Response<Body>, Error> {
    let (parts, body) = response.into_parts();
    let body_bytes = body.collect().await?.to_bytes();
    let resp = Response::from_parts(parts, Body::from(body_bytes.to_vec()));

    Ok(resp)
}
