mod auth;
mod components;
mod config;
mod db;
mod result;
mod routes;

use crate::auth::LoginRequired;
use crate::config::{Config, Env};
use crate::routes::auth::{login_form_route, login_route, logout_route};
use crate::routes::index::index_route;
use crate::routes::link::{create_link_route, delete_link_route};
use crate::routes::new::new_route;
use axum::middleware::from_extractor;
use axum::routing::{delete, get, post};
use axum::{Extension, Router};
use eyre::{Context, Result};
use log::info;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::str::FromStr;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::compression::CompressionLayer;
use tower_http::services::ServeDir;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> Result<()> {
    let env = Env::read()?;
    let config = Config::from_env(&env);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "linkstore=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let dbc = PgPoolOptions::new()
        .max_connections(env.database_pool_size.unwrap_or(5))
        .connect(&env.database_url)
        .await?;

    let app = Router::new()
        .route("/login", get(login_form_route))
        .route("/login", post(login_route))
        .route("/logout", get(logout_route))
        .nest(
            "/",
            Router::new()
                .route("/", get(index_route))
                .route("/new", get(new_route))
                .route("/link", post(create_link_route))
                .route("/link/:id", delete(delete_link_route))
                .route_layer(from_extractor::<LoginRequired>()),
        )
        .nest_service(
            "/assets",
            ServeDir::new(&env.asset_path).precompressed_gzip(),
        )
        .layer(
            ServiceBuilder::new()
                .layer(Extension(config))
                .layer(Extension(dbc))
                .layer(CookieManagerLayer::new())
                .layer(CompressionLayer::new()),
        );

    let addr = env
        .bind
        .unwrap_or_else(|| SocketAddr::from_str("127.0.0.1:8000").unwrap());
    let listener = TcpListener::bind(&addr).await?;

    info!("Starting server on {}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .context("Error starting server")?;

    Ok(())
}
