use axum::Extension;
use axum_extra::extract::Query;
use maud::{html, Markup};
use serde::Deserialize;

use crate::components::{page, style};
use crate::db::{link_count, search_links, Database, SearchLinksRow};
use crate::result::Result;
use crate::util::urlencode;

pub fn index_view(nav: Markup, link_list: Markup, pagination: Markup) -> Markup {
    html! {
        div {
            (nav)
            (link_list)
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

fn nav(q: Option<&str>, total: u32) -> Markup {
    html! {
        nav {
            input
                type="text"
                placeholder="search"
                name="q"
                value=[q]
                hx-get="/"
                hx-trigger="keyup delay:1s"
                hx-target="body"
                hx-push-url="true";
            span { (total) " links" }
            a href="/new" hx-boost="true" { "New" }
            a href="/logout" { "Logout" }
            (style(r#"
                me {
                    margin-bottom: 20px;
                    display: flex;
                    align-items: center;

                    & > * {
                        display: inline-block;
                        margin-right: 16px;
                    }

                    & > style {
                        display: none;
                    }
                }
            "#))
        }
    }
}

fn link_list(query: &IndexQuery, links: Vec<SearchLinksRow>) -> Markup {
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
                            hx-delete=(format!("/link/{}{}", link.id, query.qs()))
                            hx-confirm="Are you sure you want to delete this link?"
                            hx-target="body"
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

fn pagination(query: &IndexQuery, total: u32, page_size: u32) -> Markup {
    let page = query.page.unwrap_or(1);
    let pages = ((total as f64) / (page_size as f64)).ceil() as u32;
    let first_page = if page > 1 {
        Some(format!("/{}", query.qs_for_page(1)))
    } else {
        None
    };
    let prev_page = if page > 1 {
        Some(format!("/{}", query.qs_for_page(page - 1)))
    } else {
        None
    };
    let next_page = if page < pages {
        Some(format!("/{}", query.qs_for_page(page + 1)))
    } else {
        None
    };
    let last_page = if page < pages {
        Some(format!("/{}", query.qs_for_page(pages)))
    } else {
        None
    };

    html! {
        div hx-boost="true" {
            @if let Some(first_page) = first_page {
                span { a href=(first_page) { "«" } }
            }
            @if let Some(prev_page) = prev_page {
                span { a href=(prev_page) { "‹" } }
            }
            span { (page) }
            @if let Some(next_page) = next_page {
                span { a href=(next_page) { "›" } }
            }
            @if let Some(last_page) = last_page {
                span { a href=(last_page) { "»" } }
            }
            (style(r#"
                me {
                    display: flex;
                    justify-content: center;

                    & > span {
                      padding: 0 6px;
                    }
                }
            "#))
        }
    }
}

const PAGE_SIZE: u32 = 20;

#[derive(Deserialize)]
pub struct IndexQuery {
    q: Option<String>,
    page: Option<u32>,
}

impl IndexQuery {
    fn qs(&self) -> String {
        self.qs_for_page(self.page.unwrap_or(1))
    }

    fn qs_for_page(&self, page: u32) -> String {
        let mut args = Vec::new();
        if let Some(q) = &self.q {
            args.push(format!("q={}", urlencode(q)));
        }
        if page > 1 {
            args.push(format!("page={}", page));
        }
        if args.is_empty() {
            "".to_string()
        } else {
            format!("?{}", &args.join("&"))
        }
    }
}

pub async fn index(dbc: &Database, query: IndexQuery) -> Result<Markup> {
    let q = query.q.as_deref();
    let total = link_count(dbc, q).await?;
    let links = search_links(dbc, q, query.page.unwrap_or(1), PAGE_SIZE).await?;
    Ok(page(index_view(
        nav(q, total),
        link_list(&query, links),
        pagination(&query, total, PAGE_SIZE),
    )))
}

pub async fn index_route(
    Extension(dbc): Extension<Database>,
    Query(query): Query<IndexQuery>,
) -> Result<Markup> {
    index(&dbc, query).await
}
