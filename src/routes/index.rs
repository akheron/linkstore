use axum::Extension;
use axum_extra::extract::Query;
use maud::{html, Markup};
use serde::Deserialize;

use crate::components::{nav, page, pagination, style};
use crate::db::{link_count, search_links, Database, SearchLinksRow};
use crate::result::Result;

#[derive(Deserialize)]
pub struct IndexQuery {
    q: Option<String>,
    page: Option<u32>,
}

pub fn index(links: Vec<SearchLinksRow>, nav: Markup, pagination: Markup) -> Markup {
    html! {
        div {
            (nav)
            (link_list(links))
            (pagination)
            (style(r#"
                me {
                  max-width: 800px;
                  margin: 0 auto;
                }
            "#))
        }
    }
}

fn link_list(links: Vec<SearchLinksRow>) -> Markup {
    html! {
        div {
            @for link in links {
                div .link {
                    a href=(link.href) target="_blank" rel="noopener noreferrer" {
                        (link.description)
                    }
                    @if !link.tags.is_empty() {
                        div {
                            @for tag in &link.tags {
                                span .tag { (tag) }
                            }
                        }
                    }
                    div {
                        span .time {
                            (link.time.format("%b %-d %Y %H:%M"))
                        }
                        " "
                        button
                            type="button"
                            hx-delete=(format!("/link/{}", link.id))
                            hx-confirm="Are you sure you want to delete this link?"
                            hx-target="closest .link" hx-swap="delete"
                        {
                            "Delete"
                        }
                    }
                }
            }
            (style(r#"
                me .link {
                    font-size: 20px;
                    padding-bottom: 16px;
    
                    & a {
                        color: #1111aa;
                        text-decoration: none;
                    }
    
                    & .tag {
                        color: #aa5511;
                        display: inline-block;
                        margin-right: 10px;
                    }
    
                    & .time {
                        color: #777777;
                    }
    
                    & form {
                        display: inline-block;
                        margin: 0 0 0 8px;
                    }
    
                    & button {
                        position: relative;
                        top: -2px;
                        background: #f0f0f0;
                        border: 1px solid black;
                        border-radius: 3px;
                        color: #000000;
                        font-size: 12px;
                        padding: 3px;
                    }
                }
            "#))
        }
    }
}

const PAGE_SIZE: u32 = 20;

pub async fn index_route(
    Extension(dbc): Extension<Database>,
    query: Query<IndexQuery>,
) -> Result<Markup> {
    let q = query.q.as_deref();
    let total = link_count(&dbc, q).await?;
    let links = search_links(&dbc, q, query.page.unwrap_or(1), PAGE_SIZE).await?;
    Ok(page(index(
        links,
        nav(query.q.clone(), total),
        pagination(q, query.page.unwrap_or(1), total, PAGE_SIZE),
    )))
}
