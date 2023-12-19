use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::header::LOCATION;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use cookie::time::Duration;
use cookie::{Cookie, SameSite};
use std::convert::Infallible;
use std::time::{SystemTime, UNIX_EPOCH};
use tower_cookies::Cookies;

use crate::config::Config;

static COOKIE_NAME: &str = "session";

pub fn login(config: &Config, cookies: &Cookies) {
    let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut cookie = Cookie::new(COOKIE_NAME, since_epoch.as_secs().to_string());
    cookie.set_path("/");
    cookie.set_same_site(SameSite::Lax);
    cookie.set_http_only(true);
    if config.env == "prod" {
        cookie.set_secure(true);
    }
    cookie.set_max_age(Some(Duration::days(14)));
    cookies.signed(&config.cookie_secret).add(cookie);
}

pub fn logout(config: &Config, cookies: &Cookies) {
    let signed = cookies.signed(&config.cookie_secret);
    let cookie_opt = signed.get(COOKIE_NAME);
    if let Some(mut cookie) = cookie_opt {
        cookie.set_path("/");
        cookies.signed(&config.cookie_secret).remove(cookie);
    }
}

pub struct IsLoggedIn(pub bool);

#[async_trait]
impl<S> FromRequestParts<S> for IsLoggedIn
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(config) = Extension::<Config>::from_request_parts(parts, state)
            .await
            .unwrap();
        let Extension(cookies) = Extension::<Cookies>::from_request_parts(parts, state)
            .await
            .unwrap();
        let cookie = cookies.signed(&config.cookie_secret).get(COOKIE_NAME);
        if cookie.is_none() {
            return Ok(Self(false));
        }
        let cookie_value = cookie.unwrap().value().parse::<u64>();
        if cookie_value.is_err() {
            return Ok(Self(false));
        }

        let seconds_since_epoch = cookie_value.unwrap();
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        if time.as_secs() - seconds_since_epoch > 60 * 60 * 24 * 14 {
            return Ok(Self(false));
        }

        Ok(Self(true))
    }
}

pub struct LoginRequired;

#[async_trait]
impl<S> FromRequestParts<S> for LoginRequired
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let IsLoggedIn(is_logged_in) = IsLoggedIn::from_request_parts(parts, state).await.unwrap();
        if is_logged_in {
            Ok(Self)
        } else {
            let pq = parts
                .uri
                .path_and_query()
                .map(|pq| pq.as_str())
                .unwrap_or("/");
            let next = url::form_urlencoded::byte_serialize(pq.as_bytes()).collect::<String>();
            Err((
                StatusCode::SEE_OTHER,
                [(LOCATION, format!("/login?next={}", next))],
            )
                .into_response())
        }
    }
}
