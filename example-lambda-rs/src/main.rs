use example_lambda_rs::handler::handle;
use lambda_runtime::{run, service_fn, tracing, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(handle)).await
}
