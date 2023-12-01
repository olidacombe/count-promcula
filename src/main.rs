use axum::{routing::get, Router};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::{future::ready, time::Duration};
use tokio::time::sleep;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use count_promcula::cli::{self, Cli};

fn metrics_app() -> Router {
    let recorder_handle = setup_metrics_recorder();
    Router::new().route("/metrics", get(move || ready(recorder_handle.render())))
}

async fn start_count_task(seconds_to_live: Option<u16>) {
    let mut count: u16 = 0;
    if let Some(limit) = seconds_to_live {
        tracing::debug!("Staying alive for {limit} seconds");
    }

    loop {
        sleep(Duration::from_secs(1)).await;
        metrics::increment_counter!("alive_seconds");
        if let Some(limit) = seconds_to_live {
            if count >= limit {
                break;
            }
            count += 1;
            tracing::trace!("Been alive for {count}s")
        }
    }
}

async fn start_metrics_server(address: &str) {
    let app = metrics_app();

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    let listening_on = listener.local_addr().unwrap();
    tracing::debug!("listening on {listening_on}");
    tracing::info!("Query Prometheus on http://{listening_on}/metrics");
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

    let Cli {
        seconds,
        address,
        port,
    } = cli::parse().unwrap();

    let address = format!("{address}:{port}");
    tokio::select! {
        _ = start_count_task(seconds) => {}
        _ = start_metrics_server(&address) => {}
    };
}

fn setup_metrics_recorder() -> PrometheusHandle {
    PrometheusBuilder::new().install_recorder().unwrap()
}
