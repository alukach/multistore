use s3s::{S3Error, S3ErrorCode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Data source not found: {0}")]
    DataSourceNotFound(String),

    #[error(transparent)]
    ObjectStoreError(#[from] object_store::Error),
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

impl From<Error> for S3Error {
    fn from(error: Error) -> Self {
        match error {
            Error::ObjectStoreError(object_store::Error::NotFound { .. }) => {
                S3Error::new(S3ErrorCode::NoSuchKey)
            }
            Error::ObjectStoreError(_) => S3Error::new(S3ErrorCode::InternalError), // TODO: handle other ObjectStoreerrors
            Error::DataSourceNotFound(_) => S3Error::new(S3ErrorCode::NoSuchBucket),
        }
    }
}
