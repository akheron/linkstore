use cookie::Key;
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize)]
pub struct Env {
    pub env: String,
    pub database_url: String,
    pub database_pool_size: Option<u32>,
    pub bind: Option<SocketAddr>,
    pub cookie_secret: String,
    pub auth_username: String,
    pub auth_password: String,
    pub asset_path: String,
}

impl Env {
    pub fn read() -> eyre::Result<Env> {
        Ok(envy::from_env::<Self>()?)
    }
}

#[derive(Clone)]
pub struct Config {
    pub env: String,
    pub username: String,
    pub password: String,
    pub cookie_secret: Key,
}

impl Config {
    pub fn from_env(env: &Env) -> Self {
        Self {
            env: env.env.clone(),
            username: env.auth_username.clone(),
            password: env.auth_password.clone(),
            cookie_secret: Key::from(env.cookie_secret.as_bytes()),
        }
    }
}
