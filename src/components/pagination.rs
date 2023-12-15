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
    pub fn new(page: u32, total: u32, page_size: u32) -> Self {
        let pages = ((total as f64) / (page_size as f64)).ceil() as u32;
        Self {
            page,
            first_page: if page > 1 { Some(page_url(1)) } else { None },
            prev_page: if page > 1 {
                Some(page_url(page - 1))
            } else {
                None
            },
            next_page: if page < pages {
                Some(page_url(page + 1))
            } else {
                None
            },
            last_page: if page < pages {
                Some(page_url(pages))
            } else {
                None
            },
        }
    }
}

fn page_url(page: u32) -> String {
    if page == 1 {
        "/".to_string()
    } else {
        format!("/?page={page}")
    }
}
