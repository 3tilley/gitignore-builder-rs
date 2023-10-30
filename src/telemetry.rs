use std::ops::Sub;
// use log::Level;
use opentelemetry::KeyValue;
use opentelemetry::global;
// use opentelemetry_api::trace::Tracer;
// use opentelemetry_api::KeyValue;
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_otlp::HasExportConfig;
use opentelemetry_sdk::logs::{Config, LoggerProvider};
use opentelemetry_sdk::trace::Tracer;
use opentelemetry_sdk::trace::{BatchConfig, RandomIdGenerator, Sampler, TracerProvider};
use opentelemetry_sdk::{runtime, Resource};
use opentelemetry_semantic_conventions::{
    resource::{DEPLOYMENT_ENVIRONMENT, SERVICE_NAME, SERVICE_VERSION},
    SCHEMA_URL,
};
use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::fmt::format::{FmtSpan, Format, Pretty};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

// pub fn prepare_logging() {
//     let exporter = opentelemetry_stdout::LogExporterBuilder::default()
//         // Comment / uncomment the below lines to toggle pretty print output.
//         .with_encoder(|writer, data| Ok(serde_json::to_writer_pretty(writer, &data).unwrap()))
//         .build();
//     let logger_provider = LoggerProvider::builder()
//         .with_config(
//             Config::default().with_resource(Resource::new(vec![KeyValue::new(
//                 "service.name",
//                 "logs-basic-example",
//             )])),
//         )
//         .with_simple_exporter(exporter)
//         .build();
//
//     // Setup Log Appender for the log crate.
//     let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
//     log::set_boxed_logger(Box::new(otel_log_appender)).unwrap();
//     log::set_max_level(Level::Trace.to_level_filter());
// }

// Create a Resource that captures information about the entity for which telemetry is recorded.
fn resource() -> Resource {
    Resource::from_schema_url(
        [
            KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
            KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
            KeyValue::new(DEPLOYMENT_ENVIRONMENT, "develop"),
        ],
        SCHEMA_URL,
    )
}

pub fn get_subscriber(env_filter: String) -> impl Subscriber + Send + Sync {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let subs = tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        // .compact()
        .pretty()
        .with_file(true)
        // .with_span_events(FmtSpan::ACTIVE)
        .finish();
    subs
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    set_global_default(subscriber).expect("Failed to set subscriber")
}

// pub fn get_jaeger_subscriber(env_filter: String) -> impl Subscriber + Send + Sync {
//     let env_filter = EnvFilter::try_from_default_env()
//         .unwrap_or_else(|_| EnvFilter::new(env_filter));
// }

fn init_tracer() -> Tracer {
    let mut exporter = opentelemetry_otlp::new_exporter().http();
    let endpoint = &exporter.export_config().endpoint;
    let pipeline = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(
            opentelemetry_sdk::trace::Config::default()
                // Customize sampling strategy
                .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
                    1.0,
                ))))
                // If export trace to AWS X-Ray, you can use XrayIdGenerator
                .with_id_generator(RandomIdGenerator::default())
                .with_resource(resource()),
        )
        .with_batch_config(BatchConfig::default())
        .with_exporter(exporter);
    let original = pipeline
        // .install_simple()
        .install_batch(opentelemetry_sdk::runtime::Tokio) // This appears to need a runtime feature in otel_sdk
        .unwrap();

    // original

    // let tracer = opentelemetry_otlp::new_pipeline()
    //     .tracing()
    //     .with_exporter(
    //         opentelemetry_otlp::new_exporter()
    //             .tonic()
    //             .with_endpoint("http://localhost:4317")
    //             .with_timeout(Duration::from_secs(3))
    //             .with_metadata(map)
    //     )
    //     .with_trace_config(
    //         trace::config()
    //             .with_sampler(Sampler::AlwaysOn)
    //             .with_id_generator(RandomIdGenerator::default())
    //             .with_max_events_per_span(64)
    //             .with_max_attributes_per_span(16)
    //             .with_max_events_per_span(16)
    //             .with_resource(Resource::new(vec![KeyValue::new("service.name", "example")])),
    //     )
    //     .install_batch(opentelemetry::runtime::Tokio)?;
    original
}

// pub fn make_stdout_layer() -> Layer {
//     let stdout_layer = tracing_subscriber::fmt::Layer::new().pretty().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE);
//     stdout_layer
// }
pub fn setup_otel(env_filter: String) {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let stdout_layer = tracing_subscriber::fmt::Layer::new().pretty();
    tracing_subscriber::registry()
        .with(env_filter)
        .with(OpenTelemetryLayer::new(init_tracer()))
        .with(stdout_layer)
        .init()
}

// pub fn telemetry_router() -> String {
//     let provider = global::tracer_provider();
//     // Get the subscribers in the registry
//     let reg = tracing_subscriber::registry();
//     let tonic = opentelemetry_otlp::new_exporter().tonic();
//     tonic.tr
// }