use axum::{routing::get, Router};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::{future::ready, time::Duration};
use tokio::time::sleep;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn metrics_app() -> Router {
    let recorder_handle = setup_metrics_recorder();
    Router::new().route("/metrics", get(move || ready(recorder_handle.render())))
}

async fn start_count_task() {
    // TODO die after specified period if given
    loop {
        sleep(Duration::from_secs(1)).await;
        metrics::increment_counter!("alive_seconds");
    }
}

async fn start_metrics_server() {
    let app = metrics_app();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    tracing::info!(
        "Query Prometheus on http://{}/metrics",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (_counter, _metrics_server) = tokio::join!(start_count_task(), start_metrics_server());
}

fn setup_metrics_recorder() -> PrometheusHandle {
    PrometheusBuilder::new().install_recorder().unwrap()
}
