use crate::components::style;
use maud::{html, Markup};

pub fn nav(q: Option<String>, total: u32) -> Markup {
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
