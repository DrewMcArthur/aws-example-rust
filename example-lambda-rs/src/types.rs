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

/// This is really more for demonstration than anything else.  
/// I think we can pretty confidently say this works because it compiles
#[cfg(test)]
mod tests {
    use super::*;
    use aws_lambda_events::s3::{
        S3Bucket, S3Entity, S3Object, S3RequestParameters, S3UserIdentity,
    };
    use std::collections::HashMap;

    #[test]
    fn test_metadata_from() {
        let bucket = "test-bucket";
        let key = "test-key";
        let hash = "test-hash";
        let record = S3EventRecord {
            event_version: None,
            event_source: None,
            aws_region: None,
            event_time: Default::default(),
            event_name: None,
            principal_id: S3UserIdentity { principal_id: None },
            request_parameters: S3RequestParameters {
                source_ip_address: None,
            },
            response_elements: HashMap::new(),
            s3: S3Entity {
                schema_version: None,
                configuration_id: None,
                bucket: S3Bucket {
                    name: Some(bucket.to_string()),
                    owner_identity: None,
                    arn: None,
                },
                object: S3Object {
                    key: Some(key.to_string()),
                    size: None,
                    url_decoded_key: None,
                    version_id: None,
                    e_tag: Some(hash.to_string()),
                    sequencer: None,
                },
            },
        };
        let metadata = Metadata::from(&record);
        assert_eq!(metadata.bucket, Some(bucket.to_string()));
        assert_eq!(metadata.key, Some(key.to_string()));
        assert_eq!(metadata.hash, Some(hash.to_string()));
    }
}
