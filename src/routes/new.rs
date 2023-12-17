use askama::Template;
use axum::extract::Query;
use axum::response::IntoResponse;
use serde::Deserialize;

#[derive(Template)]
#[template(path = "new.html")]
pub struct New {
    url: String,
    title: String,
    in_window: bool,
}

#[derive(Deserialize)]
pub struct NewParams {
    pub url: Option<String>,
    pub title: Option<String>,
}

pub async fn new_route(Query(params): Query<NewParams>) -> impl IntoResponse {
    let url = params.url.unwrap_or_default();
    let title = params.title.unwrap_or_default();
    let in_window = !url.is_empty() || !title.is_empty();
    New {
        url,
        title,
        in_window,
    }
}
