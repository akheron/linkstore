use crate::components::{page, style};
use axum::extract::Query;
use axum::response::IntoResponse;
use maud::{html, Markup};
use serde::Deserialize;

pub fn new(url: String, title: String, in_window: bool) -> Markup {
    html! {
        div {
            form {
                input type="hidden" name="in_window" value=(in_window);
                div {
                    input name="href" placeholder="url" value=(url);
                }
                div {
                    input name="description" placeholder="description" value=(title);
                }
                div {
                    textarea name="extended" placeholder="long description" {}
                }
                div {
                    input name="tags" placeholder="tags";
                }
                div {
                    button hx-post="/link" hx-target="#error" { "save" }
                    " "
                    @if in_window {
                        button type="button" onclick="window.close()" { "close" }
                    } @else {
                        a href="/" hx-boost="true" { "cancel" }
                    }
                }
            }
            div #error {}
            (style(r#"
                me {
                  & form > div {
                    padding-bottom: 8px;
                  }

                  & input,
                  & textarea {
                    width: 100%;
                  }

                  & textarea {
                    height: 150px;
                  }
                }

                #error {
                  color: #ff1100;
                }
            "#))
        }
    }
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
    page(new(url, title, in_window))
}
