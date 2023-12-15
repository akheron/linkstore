use askama::Template;
use axum::Extension;
use axum_extra::extract::Query;
use serde::Deserialize;

use crate::components::{Nav, Pagination};
use crate::db::{link_count, search_links, Database, SearchLinksRow};
use crate::result::Result;

#[derive(Deserialize)]
pub struct IndexQuery {
    q: Option<String>,
    page: Option<u32>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    links: Vec<SearchLinksRow>,
    nav: Nav,
    pagination: Pagination,
}

const PAGE_SIZE: u32 = 20;

pub async fn index_route(
    Extension(dbc): Extension<Database>,
    query: Query<IndexQuery>,
) -> Result<IndexTemplate> {
    let total = link_count(&dbc, query.q.as_deref()).await?;
    let links = search_links(&dbc, query.q.as_deref(), query.page.unwrap_or(1), PAGE_SIZE).await?;
    Ok(IndexTemplate {
        links,
        nav: Nav::new(query.q.clone(), total),
        pagination: Pagination::new(query.page.unwrap_or(1), total, PAGE_SIZE),
    })
}
