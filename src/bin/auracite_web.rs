#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate auracite;
extern crate rocket;

use auracite::{web, lodestone};
use auracite::env::load_var;
use rocket::config::{Config};

fn main() {
    rocket::custom(config(), true)
        .mount("/", routes![web::web_root, web::static_asset])
        .mount("/lodestone", routes![lodestone::index, lodestone::rss])
        .catch(errors![web::not_found])
        .launch();
}

fn config() -> Config {
    use rocket::config::*;

    let env = Environment::active().unwrap();
    let mut config = Config::build(env);
    let port = load_var("PORT");
    if let Some(port) = port.parse().ok() {
        config = config.port(port);
    }

    config.finalize().unwrap()
}
