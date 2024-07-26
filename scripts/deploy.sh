#!/bin/bash -e

# Steps:
# 0. ./scripts/check-workflow.sh & run tests
# 1. Compile the rust code into a .zip of the executable binary
# 2. Create a bucket (if it doesn't already exist) to upload this binary
# 3. Package our template, which uploads the referenced local zip to our new artifacts bucket and outputs a new template
# 4. Deploy the packaged template, which refers to the uploaded zip in the artifacts bucket

set -e

echo "Checking the code..."
# ./scripts/check.sh
cargo test

# Build the release version of the Rust Lambda
echo "Building the release version of the Rust Lambda..."
cargo install cargo-lambda || true
cargo lambda build --release --output-format zip
# note: you may have to add zig to your path.  here's what i did:
# pip install zig
# should say "already satisfied in: {path}"
# explore the path until you find `zig.exe`
# add that path to your `~/.bash_profile`, `~/.bashrc`, or wherever you do that.  here's mine:
# export PATH="/c/Users/dmcarthur/appdata/local/continuum/anaconda3/lib/site-packages/ziglang/:$PATH"
# then, after running `source ~/.bash_profile` (to refresh your path),
# running `which zig` should return the path to `zig.exe`. at this point, you can retry

ARTIFACT_BUCKET=exrs-artifacts

echo "Creating artifact bucket..."
aws s3 mb s3://${ARTIFACT_BUCKET}

echo "Packaging lambda..."
aws cloudformation package --template-file infrastructure/template.yml --output-template-file infrastructure/template-packaged.yml --s3-bucket ${ARTIFACT_BUCKET}

echo "Deploying lambda..."
aws cloudformation deploy --template-file infrastructure/template-packaged.yml --stack-name exrs --capabilities CAPABILITY_IAM

echo "Cleaning up..."
rm ./infrastructure/*-packaged.yml
