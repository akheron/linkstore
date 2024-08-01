use crate::components::style;
use crate::util::urlencode;
use maud::{html, Markup};

pub fn pagination(q: Option<&str>, page: u32, total: u32, page_size: u32) -> Markup {
    let pages = ((total as f64) / (page_size as f64)).ceil() as u32;
    let first_page = if page > 1 { Some(page_url(q, 1)) } else { None };
    let prev_page = if page > 1 {
        Some(page_url(q, page - 1))
    } else {
        None
    };
    let next_page = if page < pages {
        Some(page_url(q, page + 1))
    } else {
        None
    };
    let last_page = if page < pages {
        Some(page_url(q, pages))
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

fn page_url(q: Option<&str>, page: u32) -> String {
    let mut args = Vec::new();
    if let Some(q) = q {
        args.push(format!("q={}", urlencode(q)));
    }
    if page > 1 {
        args.push(format!("page={}", page));
    }
    if args.is_empty() {
        "/".to_string()
    } else {
        format!("/?{}", &args.join("&"))
    }
}
