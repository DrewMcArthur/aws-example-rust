use std::collections::HashMap;

use aws_lambda_events::{
    s3::{
        S3Bucket, S3Entity, S3Event, S3EventRecord, S3Object, S3RequestParameters, S3UserIdentity,
    },
    sqs::{SqsEventObj, SqsMessageObj},
};
use example_lambda_rs::handler::handle;
use lambda_runtime::{Context, LambdaEvent};

#[tokio::test]
async fn basic_handle() {
    let expected_bucket = "test-bucket";
    let expected_key = "test-key";
    let msg = generate_message(expected_bucket, expected_key);
    let res = handle(msg).await.expect("Error handling message");
    assert_eq!(res.len(), 1);
    let first = res.first().expect("No metadata found");
    assert_eq!(first.bucket, Some(expected_bucket.to_string()));
    assert_eq!(first.key, Some(expected_key.to_string()));
}

fn generate_message(bucket: &str, key: &str) -> LambdaEvent<SqsEventObj<S3Event>> {
    LambdaEvent {
        payload: SqsEventObj {
            records: vec![SqsMessageObj {
                body: S3Event {
                    records: vec![S3EventRecord {
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
                                e_tag: None,
                                sequencer: None,
                            },
                        },
                    }],
                },
                message_id: None,
                receipt_handle: None,
                md5_of_body: None,
                md5_of_message_attributes: None,
                attributes: HashMap::new(),
                message_attributes: HashMap::new(),
                event_source_arn: None,
                event_source: None,
                aws_region: None,
            }],
        },
        context: Context::default(),
    }
}
