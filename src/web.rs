use std::option::Option;
use std::path::{PathBuf};

use html::{CSS, JS, Meta};
use maud::{DOCTYPE, Markup};
use rocket::response::Content;
use rocket::http::ContentType;

#[get("/")]
pub fn web_root() -> Markup {
    let content = html!{
        div.ac-layout-transparent.mdl-layout.mdl-js-layout {
            header.mdl-layout__header.mdl-layout__header--transparent {
                div.mdl-layout__header-row {
                    span.mdl-layout-title "Auracite"
                    div.mdl-layout-spacer {}
                    nav.mdl-navigation {
                        a.mdl-navigation__link href="#" "Lodestone"
                    }
                }
            }
            main.mdl-layout__content {
                "Some content"
            }
        }
    };

    let styles = html!{
        style type="text/css" { (include_str!("res/hero.css")) }
    };
    page(&content, Some(styles))
}

#[get("/static/<file..>")]
pub fn static_asset(file: PathBuf) -> Option<Content<&'static str>> {
    match file.to_str() {
        Some("hero.svg") => Some(Content(ContentType::SVG, include_str!("res/hero.svg"))),
        _ => None
    }
}

fn page(content: &Markup, extra_styles: Option<Markup>) -> Markup {
    html!{
        (DOCTYPE)
        html {
            head {
                title "Auracite"
                (Meta("viewport", "width=device-width, initial-scale=1"))
                (CSS("https://fonts.googleapis.com/icon?family=Material+Icons"))
                (CSS("https://code.getmdl.io/1.3.0/material.indigo-pink.min.css"))
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
