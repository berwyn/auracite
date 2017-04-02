use std::option::Option;
use std::path::PathBuf;

use rocket::http::ContentType;
use rocket::response::Content;

#[get("/static/<file..>")]
pub fn static_asset(file: PathBuf) -> Option<Content<&'static str>> {
    match file.to_str() {
        Some("hero.svg") => Some(Content(ContentType::SVG, include_str!("../res/hero.svg"))),
        Some("hero.css") => Some(Content(ContentType::CSS, include_str!("../res/hero.css"))),
        _ => None
    }
}
