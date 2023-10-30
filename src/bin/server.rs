use gitignore_builder_rs::telemetry::{get_subscriber, init_subscriber, setup_otel};
use gitignore_builder_rs::{make_router, shutdown_signal};
use tracing::{info_span, Instrument};

#[tokio::main]
async fn main() -> Result<(), axum::BoxError> {

    // init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers()?;

    // let subscriber = get_subscriber("INFO".into());
    // init_subscriber(subscriber);

    setup_otel("INFO".into());

    let router = make_router();

    let port = 3210;
    println!("Starting axum server on {}", port);
    axum::Server::bind(&"0.0.0.0:3210".parse().unwrap())
        .serve(router.into_make_service())
        // .with_graceful_shutdown(shutdown_signal())?;
        .instrument(info_span!("app_start"))
        .await?;
    Ok(())
}
