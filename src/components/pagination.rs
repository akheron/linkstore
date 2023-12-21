use crate::util::urlencode;
use askama::Template;

#[derive(Template)]
#[template(path = "components/pagination.html")]
pub struct Pagination {
    page: u32,
    first_page: Option<String>,
    prev_page: Option<String>,
    next_page: Option<String>,
    last_page: Option<String>,
}

impl Pagination {
    pub fn new(q: Option<&str>, page: u32, total: u32, page_size: u32) -> Self {
        let pages = ((total as f64) / (page_size as f64)).ceil() as u32;
        Self {
            page,
            first_page: if page > 1 { Some(page_url(q, 1)) } else { None },
            prev_page: if page > 1 {
                Some(page_url(q, page - 1))
            } else {
                None
            },
            next_page: if page < pages {
                Some(page_url(q, page + 1))
            } else {
                None
            },
            last_page: if page < pages {
                Some(page_url(q, pages))
            } else {
                None
            },
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
