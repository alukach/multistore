use s3s::route::S3Route;
use s3s::{Body, S3Request, S3Response, S3Result};

use log::info;
use axum::http;
use http::{Extensions, HeaderMap, Method, StatusCode, Uri};
use std::net::SocketAddr;
use tower::Service;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

pub struct CustomRoute {
    prefix: String,
    router: axum::Router,
}

impl CustomRoute {
    #[must_use]
    pub fn build(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
            router: self::handlers::register(prefix),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Extra {
    pub credentials: Option<s3s::auth::Credentials>,
    pub region: Option<String>,
    pub service: Option<String>,
}

fn convert_request(req: S3Request<Body>) -> http::Request<Body> {
    let (mut parts, _) = http::Request::new(Body::empty()).into_parts();
    parts.method = req.method;
    parts.uri = req.uri;
    parts.headers = req.headers;
    parts.extensions = req.extensions;
    parts.extensions.insert(Extra {
        credentials: req.credentials,
        region: req.region,
        service: req.service,
    });
    http::Request::from_parts(parts, req.input)
}

fn convert_response(resp: http::Response<axum::body::Body>) -> S3Response<(StatusCode, Body)> {
    let (parts, body) = resp.into_parts();
    let mut s3_resp = S3Response::new((parts.status, Body::http_body_unsync(body)));
    // s3_resp.status = Some(parts.status);
    s3_resp.headers = parts.headers;
    s3_resp.extensions = parts.extensions;
    s3_resp
}

#[async_trait::async_trait]
impl S3Route for CustomRoute {
    fn is_match(
        &self,
        _method: &Method,
        uri: &Uri,
        _headers: &HeaderMap,
        _extensions: &mut Extensions,
    ) -> bool {
        let path = uri.path();
        println!("path: {}", path);
        let prefix = self.prefix.clone() + "/";
        path.starts_with(&prefix)
    }

    async fn check_access(&self, req: &mut S3Request<Body>) -> S3Result<()> {
        if req.credentials.is_none() {
            tracing::debug!("anonymous access");
        }
        Ok(()) // allow all requests
    }

    async fn call(&self, req: S3Request<Body>) -> S3Result<S3Response<(StatusCode, s3s::Body)>> {
        let mut service = self.router.clone().into_service::<Body>();
        let req = convert_request(req);
        let result = service.call(req).await;
        match result {
            Ok(resp) => Ok(convert_response(resp)),
            Err(e) => match e {},
        }
    }
}

mod handlers {
    use std::collections::HashMap;

    use axum::Json;
    use axum::Router;
    use axum::body::Body;
    use axum::extract::Path;
    use axum::extract::Query;
    use axum::extract::Request;
    use axum::http::Response;
    use axum::response;
    use axum::routing::get;
    use axum::routing::post;

    pub async fn echo(req: Request) -> Response<Body> {
        Response::new(req.into_body())
    }

    pub async fn hello() -> &'static str {
        "Hello, World!"
    }

    pub async fn show_path(Path(path): Path<String>) -> String {
        path
    }

    pub async fn show_query(Query(query): Query<HashMap<String, String>>) -> String {
        format!("{query:?}")
    }

    pub async fn show_json(
        Json(json): Json<serde_json::Value>,
    ) -> response::Json<serde_json::Value> {
        tracing::debug!(?json);
        response::Json(json)
    }

    pub fn register(prefix: &str) -> Router {
        let router = Router::new()
            .route("/echo", post(echo))
            .route("/hello", get(hello))
            .route("/show_path/{*path}", get(show_path))
            .route("/show_query", get(show_query))
            .route("/show_json", post(show_json));

        Router::new().nest(prefix, router)
    }
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .pretty()
        // This allows you to use, e.g., `RUST_LOG=info` or `RUST_LOG=debug`
        // when running the app to set log levels.
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("axum_tracing_example=error,tower_http=warn"))
                .unwrap(),
        )
        .init();

    // build our application with a route
    let custom_route = CustomRoute::build("/foo");

    // run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(
        listener,
        custom_route.router.layer(TraceLayer::new_for_http()),
    )
    .await
    .unwrap();
}
