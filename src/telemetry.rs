use opentelemetry_sdk::logs::{Config, LoggerProvider};
use opentelemetry_sdk::Resource;
use opentelemetry::KeyValue;
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use log::Level;

pub fn prepare_logging() {
    let exporter = opentelemetry_stdout::LogExporterBuilder::default()
        // Comment / uncomment the below lines to toggle pretty print output.
        .with_encoder(|writer, data|
           Ok(serde_json::to_writer_pretty(writer, &data).unwrap()))
        .build();
    let logger_provider = LoggerProvider::builder()
        .with_config(
            Config::default().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                "logs-basic-example",
            )])),
        )
        .with_simple_exporter(exporter)
        .build();

    // Setup Log Appender for the log crate.
    let otel_log_appender = OpenTelemetryLogBridge::new(&logger_provider);
    log::set_boxed_logger(Box::new(otel_log_appender)).unwrap();
    log::set_max_level(Level::Trace.to_level_filter());
}
