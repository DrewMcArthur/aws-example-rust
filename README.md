# AWS Example (Rust)

This repository is designed to be an example codebase
to run some rust code inside lambdas on AWS.

Deploying this cloudformation stack will create
a bucket, a queue, and a lambda.  
When you update the bucket (e.g. adding or removing a file),
it will trigger an sqs message, which triggers the lambda.
The lambda logs the event and parses it a little bit.

## Setup

You'll need:

- [Rust](https://www.rust-lang.org/), to install:
  `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

The first time you run either of the bash scripts,
some other dependencies will be installed.

## Development

- Build: `cargo build`
- Test: `cargo test`
- Deploy: `bash ./scripts/deploy.sh`
- Check CI workflow: `bash ./scripts/check.sh`

## Additional Resources

- [cargo-lambda.info](https://www.cargo-lambda.info) a CLI tool for Rust Lambdas
- [rust-problem-matchers](https://github.com/r7kamura/rust-problem-matchers) (allows build errors to show on PR diffs)
- [Ectobit's Rust CI setup](https://ectobit.com/blog/speed-up-github-actions-rust-pipelines/) - a walkthrough on speeding up CI
- [Deployment workflow](https://stackoverflow.com/questions/47008234/cloudformation-lambda-function-give-local-code) - SO question on packaging code via AWS CF
- [aws-sdk-rs documentation](https://aws.amazon.com/sdk-for-rust/)
- Some AWS example codebases
  - [aws-lambda-rust-runtime examples](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples)
  - [aws-samples/serverless-rust-demo](https://github.com/aws-samples/serverless-rust-demo/)
