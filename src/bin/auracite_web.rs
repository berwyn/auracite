#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate auracite;
extern crate rocket;

use auracite::{web, lodestone};

fn main() {
    rocket::ignite()
        .mount("/", routes![web::web_root, web::static_asset])
        .mount("/lodestone", routes![lodestone::index, lodestone::rss])
        .launch();
}
