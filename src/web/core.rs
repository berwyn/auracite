use std::borrow::Cow;

use rocket::http::ContentType;
use rocket::response::Content;

static_resource!(get_file, "./src/res/404.html", "../res/404.html");

#[error(404)]
pub fn not_found() -> Content<String> {
    let file = get_file();
    let str = String::from_utf8(file.to_vec()).unwrap();
    Content(ContentType::HTML, str)
}
