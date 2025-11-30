use chrono::{DateTime, Utc};
use object_store::ObjectMeta;
use s3s::dto;
use std::time::{SystemTime, UNIX_EPOCH};

/// A wrapper around ObjectMeta that provides conversion to/from dto::Object
#[derive(Debug, Clone)]
pub struct S3ObjectMeta(ObjectMeta);

impl S3ObjectMeta {
    /// Create a new S3ObjectMeta from an ObjectMeta
    pub fn new(meta: ObjectMeta) -> Self {
        Self(meta)
    }

    /// Get a reference to the inner ObjectMeta
    pub fn inner(&self) -> &ObjectMeta {
        &self.0
    }

    /// Convert into the inner ObjectMeta
    pub fn into_inner(self) -> ObjectMeta {
        self.0
    }
}

impl From<ObjectMeta> for S3ObjectMeta {
    fn from(meta: ObjectMeta) -> Self {
        Self(meta)
    }
}

impl From<S3ObjectMeta> for ObjectMeta {
    fn from(meta: S3ObjectMeta) -> Self {
        meta.0
    }
}

#[derive(Debug, Clone)]
pub struct Timestamp(SystemTime);

impl From<dto::Timestamp> for Timestamp {
    fn from(timestamp: dto::Timestamp) -> Self {
        Self(std::time::SystemTime::from(time::OffsetDateTime::from(
            timestamp,
        )))
    }
}

impl From<Timestamp> for dto::Timestamp {
    fn from(timestamp: Timestamp) -> Self {
        dto::Timestamp::from(timestamp.0)
    }
}

impl From<DateTime<Utc>> for Timestamp {
    fn from(timestamp: DateTime<Utc>) -> Self {
        Self(std::time::SystemTime::from(timestamp))
    }
}

impl From<&dto::Object> for S3ObjectMeta {
    fn from(object: &dto::Object) -> Self {
        let last_modified = object
            .last_modified
            .clone()
            .map(|ts| Timestamp::from(ts).0)
            .unwrap_or(UNIX_EPOCH);

        Self(ObjectMeta {
            location: object_store::path::Path::from(object.key.as_deref().unwrap_or("")),
            last_modified: last_modified.into(),
            size: object.size.unwrap_or(0) as u64,
            e_tag: object.e_tag.as_ref().map(|etag| match etag {
                dto::ETag::Strong(value) => format!("\"{}\"", value),
                dto::ETag::Weak(value) => format!("W/\"{}\"", value),
            }),
            version: None,
        })
    }
}

impl From<S3ObjectMeta> for dto::Object {
    fn from(meta: S3ObjectMeta) -> Self {
        Self {
            key: Some(meta.0.location.to_string()),
            size: Some(meta.0.size as i64),
            last_modified: Some(Timestamp::from(meta.0.last_modified).into()),
            e_tag: meta.0.e_tag.map(parse_etag),
            ..Default::default()
        }
    }
}

// For convenience, implement AsRef and AsMut
impl AsRef<ObjectMeta> for S3ObjectMeta {
    fn as_ref(&self) -> &ObjectMeta {
        &self.0
    }
}

impl AsMut<ObjectMeta> for S3ObjectMeta {
    fn as_mut(&mut self) -> &mut ObjectMeta {
        &mut self.0
    }
}

pub trait Convert<T> {
    fn convert(self) -> T;
}

impl<T, U> Convert<U> for T
where
    T: Into<U>,
{
    fn convert(self) -> U {
        self.into()
    }
}

pub fn parse_etag(s: String) -> dto::ETag {
    if let Some(stripped) = s.strip_prefix("W/\"").and_then(|s| s.strip_suffix('"')) {
        dto::ETag::Weak(stripped.to_string())
    } else if let Some(stripped) = s.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        dto::ETag::Strong(stripped.to_string())
    } else {
        dto::ETag::Strong(s)
    }
}
