use anyhow::{Ok, Result};
use ava_bot::handlers::index_page;
use axum::{routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use clap::Parser;
use std::sync::Arc;
use tracing::info;

#[derive(Debug, Parser)]
#[clap(name = "ava")]
struct Args {
    #[clap(short, long, default_value = "8080")]
    port: u16,

    #[clap(short, long, default_value = ".certs")] // 更改此处
    cert_path: String,
}

#[derive(Debug, Default)]
pub(crate) struct AppState {}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let state = Arc::new(AppState::default());
    let app = Router::new()
        .route("/", get(index_page))
        .nest_service("/public", ServeDir::new("/public"))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", args.port);
    info!("Listen on {}", addr);

    let cert = std::fs::read(format!("{}/cert.pem", args.cert_path))?;
    let key = std::fs::read(format!("{}/key.pem", args.cert_path))?;

    let config = RustlsConfig::from_pem(cert, key).await?;
    axum_server::bind_rustls(addr.parse()?, config)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
