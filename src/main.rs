mod components;
mod db;
mod result;
mod routes;

use crate::routes::index::index_route;
use crate::routes::link::{create_link_route, delete_link_route};
use crate::routes::new::new_route;
use axum::routing::{delete, get, post};
use axum::{Extension, Router};
use eyre::{Context, Result};
use log::info;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "linkstore=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let dbc = PgPoolOptions::new()
        .max_connections(10)
        .connect("postgres://linkstore:linkstore@localhost:5432/linkstore")
        .await?;

    let assets_path = std::env::current_dir().unwrap();
    let app = Router::new()
        .route("/", get(index_route))
        .route("/new", get(new_route))
        .route("/link", post(create_link_route))
        .route("/link/:id", delete(delete_link_route))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())).precompressed_gzip(),
        )
        .layer(
            ServiceBuilder::new()
                .layer(Extension(dbc))
                .layer(CompressionLayer::new()),
        );

    let bind = std::env::var("BIND").unwrap_or_else(|_| "127.0.0.1:8000".to_string());
    let addr = SocketAddr::from_str(&bind)?;
    let listener = TcpListener::bind(&addr).await?;

    info!("Starting server on {}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .context("Error starting server")?;

    Ok(())
}
