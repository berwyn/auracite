#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate auracite;
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/lodestone", routes![auracite::lodestone::index, auracite::lodestone::rss])
        .launch();
}
