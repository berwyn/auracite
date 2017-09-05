#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate auracite;
extern crate rocket;

use auracite::{web, lodestone};
use auracite::env::load_var;
use rocket::config::Config;

fn main() {
    rocket::custom(config(), true)
        .mount("/", routes![web::root::web_root, web::assets::static_asset])
        .mount("/lodestone", routes![lodestone::rss, lodestone::jsonfeed])
        .catch(errors![web::core::not_found])
        .launch();
}

fn config() -> Config {
    use rocket::config::*;

    let env = Environment::active().unwrap();
    let mut config = Config::build(env);
    config = config.address("0.0.0.0");

    let port = load_var("PORT", "8080");
    if let Some(port) = port.parse().ok() {
        config = config.port(port);
    }

    config.finalize().unwrap()
}
