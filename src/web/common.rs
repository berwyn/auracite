use html::{CSS, JS, Meta};
use maud::{DOCTYPE, Markup};

pub fn page(content: &Markup, extra_styles: Option<Markup>) -> Markup {
    html!{
        (DOCTYPE)
        html {
            head {
                title "Auracite"
                (Meta("viewport", "width=device-width, initial-scale=1"))
                (CSS("https://fonts.googleapis.com/icon?family=Material+Icons"))
                (CSS("https://code.getmdl.io/1.3.0/material.deep_purple-purple.min.css"))
                (JS("https://code.getmdl.io/1.3.0/material.min.js", true))
                @if extra_styles.is_some() {
                    (extra_styles.unwrap())
                }
            }
            body {
                (content)
            }
        }
    }
}
