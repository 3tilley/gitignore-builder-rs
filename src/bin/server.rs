use gitignore_builder_rs::{make_router, shutdown_signal};
use gitignore_builder_rs::telemetry::prepare_logging;

#[tokio::main]
async fn main() -> Result<(), axum::BoxError> {
    prepare_logging();
    init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;
    let router = make_router();

    axum::Server::bind(&"0.0.0.0:3210".parse().unwrap())
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())?;
    Ok(())
}
