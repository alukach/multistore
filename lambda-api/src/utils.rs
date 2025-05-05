use http_body_util::BodyExt;
use lambda_http::Error;
use lambda_http::{Body, Request, Response};
use lib::error::Result;

pub async fn convert_request(req_in: Request) -> Result<hyper::Request<s3s::Body>, Error> {
    // Clone necessary parts before moving
    let method = req_in.method().clone();
    let uri = req_in.uri().clone();
    let headers = req_in.headers().clone();

    // Convert lambda_http::Body to hyper::body::Bytes
    let body = req_in.into_body();
    let bytes = body.collect().await?.to_bytes();

    // Create a new hyper::Request with the converted body
    let req_out = {
        let mut req = hyper::Request::new(bytes.into());
        *req.method_mut() = method;
        *req.uri_mut() = uri;
        *req.headers_mut() = headers;
        req
    };
    println!("req_out: {:?}", req_out);

    Ok(req_out)
}

pub async fn convert_response(
    response: hyper::Response<s3s::Body>,
) -> Result<Response<Body>, Error> {
    let (parts, body) = response.into_parts();
    let body_bytes = body.collect().await?.to_bytes();
    let resp = Response::from_parts(parts, Body::from(body_bytes.to_vec()));

    Ok(resp)
}
