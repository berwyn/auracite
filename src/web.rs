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
            header.mdl-layout__header {
                div.mdl-layout__header-row {
                    span.mdl-layout-title "Auracite"
                    div.mdl-layout-spacer {}
                    nav.mdl-navigation {
                        a.mdl-navigation__link href="/lodestone" "Lodestone"
                    }
                }
            }
            div.mdl-layout__drawer {
                span.mdl-layout-title "Auracite"
                nav.mdl-navigation {
                    a.mdl-navigation__link href="/lodestone" "Lodestone"
                }
            }
            main.mdl-layout__content {
                div.page-content {
                    (hero())
                }
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
        Some("hero.css") => Some(Content(ContentType::CSS, include_str!("res/hero.css"))),
        _ => None
    }
}

#[error(404)]
pub fn not_found() -> Markup {
    html!{
        h1 "404"
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

fn hero() -> Markup {
    html!{
        div.hero {
            img src="http://placehold.it/250"
            h1 "Auracite"
        }
    }
}
