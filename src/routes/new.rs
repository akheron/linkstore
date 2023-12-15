use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "new.html")]
pub struct New;

pub async fn new_route() -> impl IntoResponse {
    New.into_response()
}
