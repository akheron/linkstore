use maud::{html, Markup, PreEscaped};

pub fn style(text: &str) -> Markup {
    html! {
        style {
            (PreEscaped(text))
        }
    }
}
