#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate xml;

mod lodestone;
mod rss;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/lodestone", routes![lodestone::index, lodestone::rss])
        .launch();
}
