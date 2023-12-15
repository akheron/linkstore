use askama::Template;

#[derive(Template)]
#[template(path = "components/nav.html")]
pub struct Nav {
    q: String,
    total: u32,
}

impl Nav {
    pub fn new(q: Option<String>, total: u32) -> Self {
        Self {
            q: q.unwrap_or_default(),
            total,
        }
    }
}
