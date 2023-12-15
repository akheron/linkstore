use askama_axum::{IntoResponse, Response};
use axum::extract::{Form, Path};
use axum::http::HeaderName;
use axum::Extension;
use serde::Deserialize;
use url::Url;

use crate::db::{create_link, delete_link, CreateLink, Database};
use crate::result::Result;

#[derive(Deserialize, Debug)]
pub struct CreateLinkBody {
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
    Ok(CreateLinkStatus::Created)
}

pub enum CreateLinkStatus {
    Created,
    Error(&'static str),
}

impl IntoResponse for CreateLinkStatus {
    fn into_response(self) -> Response {
        match self {
            CreateLinkStatus::Created => {
                [(HeaderName::from_static("hx-location"), "/")].into_response()
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
) -> Result<()> {
    delete_link(&dbc, id).await?;
    Ok(())
}
