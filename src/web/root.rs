use std::borrow::Cow;

use rocket::http::ContentType;
use rocket::response::Content;

static_resource!(get_file, "./src/res/index.html", "../res/index.html");

#[get("/")]
pub fn web_root() -> Content<String> {
    let file = get_file();
    let str = String::from_utf8(file.to_vec()).unwrap();
    Content(ContentType::HTML, str)
}
