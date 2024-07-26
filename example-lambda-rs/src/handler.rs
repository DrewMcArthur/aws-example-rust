use aws_lambda_events::s3::{S3Event, S3EventRecord};
use aws_lambda_events::sqs::SqsEventObj;
use lambda_runtime::{tracing, Error, LambdaEvent};

use crate::types::Metadata;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
pub async fn handle(event: LambdaEvent<SqsEventObj<S3Event>>) -> Result<Vec<Metadata>, Error> {
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
