use aws_lambda_events::s3::S3EventRecord;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Metadata {
    pub bucket: Option<String>,
    pub key: Option<String>,
    pub hash: Option<String>,
}

impl From<&S3EventRecord> for Metadata {
    fn from(record: &S3EventRecord) -> Self {
        let bucket = record.s3.bucket.name.clone();
        let key = record.s3.object.key.clone();
        let hash = record.s3.object.e_tag.clone();
        Metadata { bucket, key, hash }
    }
}
