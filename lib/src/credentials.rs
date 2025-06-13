/// Tooling to track user credentials.
use s3s::auth::Credentials;

pub mod static_auth;

#[derive(Debug, Clone)]
struct UserCredentials {
    user_id: String,
    credentials: Credentials,
}
