use crate::auth;
use crate::components::{page, style};
use crate::config::Config;
use axum::extract::Query;
use axum::http::header::LOCATION;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Form};
use maud::{html, Markup};
use serde::Deserialize;
use tower_cookies::Cookies;

pub fn login(username: &str, next: &str, login_error: bool) -> Markup {
    html! {
        form method="post" {
            input type="hidden" name="next" value=(next);
            div {
                input name="username" placeholder="username" value=(username);
            }
            div {
                input name="password" placeholder="password" type="password";
            }
            div {
                button type="submit" { "login" }
            }
            @if login_error {
                div.error { "Invalid username or password" }
            }
            (style(r#"
                me {
                    .error {
                        padding-top: 8px;
                        font-size: 16px;
                        color: red;
                    }
                }
            "#))
        }
    }
}

#[derive(Deserialize)]
pub struct LoginFormQuery {
    next: String,
}

pub async fn login_form_route(q: Option<Query<LoginFormQuery>>) -> Response {
    page(login(
        "",
        q.as_deref().map(|q| q.next.as_str()).unwrap_or("/"),
        false,
    ))
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
        auth::login(&config, &cookies);
        (StatusCode::SEE_OTHER, [(LOCATION, body.next)]).into_response()
    } else {
        page(login(&body.username, &body.next, true)).into_response()
    }
}

pub async fn logout_route(
    Extension(config): Extension<Config>,
    cookies: Cookies,
) -> impl IntoResponse {
    auth::logout(&config, &cookies);
    (StatusCode::SEE_OTHER, [(LOCATION, "/")])
}
