use s3s::auth::{Credentials, S3Auth, SecretKey};
use s3s::{S3Error, S3Result};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct UserCredentials {
    user_id: String,
    credentials: Credentials,
}

pub struct YAMLAuth {
    credentials: HashMap<String, UserCredentials>,
}

impl YAMLAuth {
    pub fn from_yaml(path: &str) -> Self {
        let file = std::fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        let config: serde_yaml::Value = serde_yaml::from_reader(reader).unwrap();
        let credentials = config["credentials"].as_sequence().unwrap();
        let credentials = credentials
            .iter()
            .map(|v| {
                let user_id = v["user_id"].as_str().unwrap().to_string();
                let access_key = v["access_key_id"].as_str().unwrap().to_string();
                let secret_key = v["secret_access_key"].as_str().unwrap().to_string();
                (
                    access_key.clone(),
                    UserCredentials {
                        user_id,
                        credentials: Credentials {
                            access_key,
                            secret_key: secret_key.into(),
                        },
                    },
                )
            })
            .collect();
        Self { credentials }
    }
}

#[async_trait::async_trait]
impl S3Auth for YAMLAuth {
    async fn get_secret_key(&self, access_key: &str) -> S3Result<SecretKey> {
        let Some(credentials) = self.credentials.get(access_key) else {
            return Err(S3Error::new(s3s::S3ErrorCode::InvalidAccessKeyId));
        };
        Ok(credentials.credentials.secret_key.clone())
    }
}
