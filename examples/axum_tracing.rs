use axum::{routing::get, Router};
use std::time::Duration;
use tracing::{debug, info, instrument, level_filters::LevelFilter, warn};
use tracing_appender::non_blocking;
use tracing_subscriber::{
    fmt::{format::FmtSpan, Layer},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer as _,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let file_appender = tracing_appender::rolling::hourly("./tmp/logs", "ecosystem.log");
    let (non_blocking, _guard) = non_blocking(file_appender);

    let console = Layer::new()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::DEBUG);

    let file_layer = Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .with_writer(non_blocking)
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(console)
        .with(file_layer)
        .init();

    let addr = "0.0.0.0:3000";
    let app = Router::new().route("/", get(index_handler));

    info!("Starting server on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[instrument]
async fn index_handler() -> &'static str {
    debug!("Handling request");
    tokio::time::sleep(Duration::from_millis(100)).await;
    let ret = long_task().await;
    info!(http_status = 200, "Request handled successfully");
    ret
}

#[instrument]
async fn long_task() -> &'static str {
    let dur = 112;
    tokio::time::sleep(Duration::from_millis(dur)).await;
    warn!(app.task_duration = dur, "Long task completed");
    "Hello, World!"
}
