use crate::auth::{login, logout};
use crate::config::Config;
use askama::Template;
use askama_axum::{IntoResponse, Response};
use axum::extract::Query;
use axum::http::header::LOCATION;
use axum::http::StatusCode;
use axum::{Extension, Form};
use serde::Deserialize;
use tower_cookies::Cookies;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate<'a> {
    username: &'a str,
    next: &'a str,
    login_error: bool,
}

#[derive(Deserialize)]
pub struct LoginFormQuery {
    next: String,
}

pub async fn login_form_route(q: Option<Query<LoginFormQuery>>) -> Response {
    LoginTemplate {
        username: "",
        next: q.as_deref().map(|q| q.next.as_str()).unwrap_or("/"),
        login_error: false,
    }
    .into_response()
}

#[derive(Deserialize)]
pub struct LoginForm {
    next: String,
    username: String,
    password: String,
}

pub async fn login_route(
    Extension(config): Extension<Config>,
    cookies: Cookies,
    Form(body): Form<LoginForm>,
) -> Response {
    if body.username == config.username && body.password == config.password {
        login(&config, &cookies);
        (StatusCode::SEE_OTHER, [(LOCATION, body.next)]).into_response()
    } else {
        (
            StatusCode::BAD_REQUEST,
            LoginTemplate {
                username: &body.username,
                next: &body.next,
                login_error: true,
            },
        )
            .into_response()
    }
}

pub async fn logout_route(
    Extension(config): Extension<Config>,
    cookies: Cookies,
) -> impl IntoResponse {
    logout(&config, &cookies);
    (StatusCode::SEE_OTHER, [(LOCATION, "/")])
}
