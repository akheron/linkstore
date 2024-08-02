use crate::db::{create_link, delete_link, CreateLink, Database};
use crate::result::Result;
use crate::routes::index::{index, IndexQuery};
use axum::extract::{Form, Path};
use axum::http::HeaderName;
use axum::response::{Html, IntoResponse, Response};
use axum::Extension;
use axum_extra::extract::Query;
use maud::Markup;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize, Debug)]
pub struct CreateLinkBody {
    in_window: bool,
    href: String,
    description: String,
    extended: String,
    tags: String,
}

pub async fn create_link_route(
    Extension(dbc): Extension<Database>,
    Form(body): Form<CreateLinkBody>,
) -> Result<CreateLinkStatus> {
    let Ok(href) = Url::parse(&body.href) else {
        return Ok(CreateLinkStatus::Error("Invalid URL"));
    };
    if body.description.is_empty() {
        return Ok(CreateLinkStatus::Error("Description cannot be empty"));
    }
    let tags = body.tags.split_whitespace().collect::<Vec<_>>();
    let created = create_link(
        &dbc,
        CreateLink {
            href,
            description: &body.description,
            extended: &body.extended,
            tags,
        },
    )
    .await?;
    if !created {
        return Ok(CreateLinkStatus::Error("Link already exists"));
    }
    Ok(CreateLinkStatus::Created(body.in_window))
}

pub enum CreateLinkStatus {
    Created(bool),
    Error(&'static str),
}

impl IntoResponse for CreateLinkStatus {
    fn into_response(self) -> Response {
        match self {
            CreateLinkStatus::Created(in_window) => {
                if in_window {
                    Html("<script>window.close()</script>").into_response()
                } else {
                    [(HeaderName::from_static("hx-location"), "/")].into_response()
                }
            }
            CreateLinkStatus::Error(msg) => msg.into_response(),
        }
    }
}

#[derive(Deserialize)]
pub struct DeleteLinkParams {
    pub id: i32,
}

pub async fn delete_link_route(
    Extension(dbc): Extension<Database>,
    Path(DeleteLinkParams { id }): Path<DeleteLinkParams>,
    Query(query): Query<IndexQuery>,
) -> Result<Markup> {
    delete_link(&dbc, id).await?;
    index(&dbc, query).await
}
