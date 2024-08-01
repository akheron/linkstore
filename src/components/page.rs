use crate::components::style;
use maud::{html, Markup, DOCTYPE};

pub fn page(content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                title { "linkstore" }
                meta charset="utf-8" {}
                meta name="viewport" content="width=device-width" {}
                (style(r#"
                    * {
                      box-sizing: border-box;
                    }
                    body {
                      font-family: 'Open Sans', sans-serif;
                      font-size: 20px;
                      margin: 8px;
                    }
                    a {
                      color: #1111aa;
                      text-decoration: none;
                    }
                "#))
                script src="/assets/htmx.2.0.1.min.js" {}
                script src="/assets/css-scope-inline.js" {}
            }
            body {
                (content)
            }
        }
    }
}
