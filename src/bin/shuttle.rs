use std::env;
use shuttle_secrets::SecretStore;
use gitignore_builder_rs::make_router;
use gitignore_builder_rs::telemetry::setup_otel;

const TRACES_ENDPOINT: &'static str = "OTEL_EXPORTER_OTLP_TRACES_ENDPOINT";

#[shuttle_runtime::main]
async fn axum_main(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let sec = secret_store.get(TRACES_ENDPOINT).expect("No OTEL config");
    env::set_var(TRACES_ENDPOINT, sec);
    setup_otel("INFO".into());
    let router = make_router();
    Ok(router.into())
}
