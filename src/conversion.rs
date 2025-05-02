use object_store::ObjectMeta;
use s3s::dto;
use std::time::SystemTime;

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

impl From<&dto::Object> for S3ObjectMeta {
    fn from(object: &dto::Object) -> Self {
        let last_modified = object
            .last_modified
            .as_ref()
            // TODO: There has to be a better way to do this conversion from s3s::dto::Timestamp to chrono::DateTime<Utc>
            .map(|ts| std::time::SystemTime::from(time::OffsetDateTime::from(ts.clone())))
            .unwrap_or(std::time::SystemTime::now());

        Self(ObjectMeta {
            location: object_store::path::Path::from(object.key.as_deref().unwrap_or("")),
            last_modified: last_modified.into(),
            size: object.size.unwrap_or(0) as u64,
            e_tag: object.e_tag.clone(),
            version: None,
        })
    }
}

impl From<S3ObjectMeta> for dto::Object {
    fn from(meta: S3ObjectMeta) -> Self {
        Self {
            key: Some(meta.0.location.to_string()),
            size: Some(meta.0.size as i64),
            last_modified: Some(dto::Timestamp::from(SystemTime::from(meta.0.last_modified))),
            e_tag: meta.0.e_tag,
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
