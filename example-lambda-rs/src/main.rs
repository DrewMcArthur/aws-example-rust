use aws_lambda_events::s3::{S3Event, S3EventRecord};
use aws_lambda_events::sqs::SqsEventObj;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Metadata {
    bucket: Option<String>,
    key: Option<String>,
    hash: Option<String>,
}

impl From<&S3EventRecord> for Metadata {
    fn from(record: &S3EventRecord) -> Self {
        let bucket = record.s3.bucket.name.clone();
        let key = record.s3.object.key.clone();
        let hash = record.s3.object.e_tag.clone();
        Metadata { bucket, key, hash }
    }
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(
    event: LambdaEvent<SqsEventObj<S3Event>>,
) -> Result<Vec<Metadata>, Error> {
    // Extract some useful information from the request
    tracing::info!("received event: {:?}", event);

    let records: Vec<S3EventRecord> = event
        .payload
        .records
        .iter()
        .flat_map(|r| r.body.records.clone())
        .collect();

    let metadata: Vec<Metadata> = records.iter().map(Metadata::from).collect();

    tracing::info!("event metadata: {:?}", metadata);
    Ok(metadata)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
