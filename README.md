# AWS Example (Rust)

This repository is designed to be an example codebase to run some rust code inside lambdas on AWS.

Deploying this cloudformation stack will create a bucket, a queue, and a lambda.  
When you update the bucket (e.g. adding a file), it will trigger an sqs message,
which triggers the lambda. The lambda simply logs the message.

## Setup

You'll need:

- Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

The first time you run either of the bash scripts, some other dependencies will be installed.

## Development

- Build: `cargo build`
- Test: `cargo test`
- Deploy: `bash ./scripts/deploy.sh`
- Check CI workflow: `bash ./scripts/check-workflow.sh`

## Additional Resources

- [cargo-lambda.info](https://www.cargo-lambda.info)
- [rust-problem-matchers](https://github.com/r7kamura/rust-problem-matchers) (allows build errors to show on PR diffs)
- [Ectobit's Rust CI setup](https://ectobit.com/blog/speed-up-github-actions-rust-pipelines/)
- [Deployment workflow](https://stackoverflow.com/questions/47008234/cloudformation-lambda-function-give-local-code)
- [aws-sdk-rs](https://aws.amazon.com/sdk-for-rust/)
- [aws-lambda-rust-runtime examples](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples)
- [aws-samples/serverless-rust-demo](https://github.com/aws-samples/serverless-rust-demo/)
