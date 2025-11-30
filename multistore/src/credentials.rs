/// Tooling to track user credentials.
use s3s::auth::Credentials;

pub mod in_memory;

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct UserCredentials {
    user_id: String,
    credentials: Credentials,
}
