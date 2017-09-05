#![feature(plugin)]
#![plugin(dotenv_macros)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

extern crate crypto;
extern crate jsonfeed;
extern crate redis;
extern crate rocket;
extern crate rss;
extern crate serde_json;

pub mod env;
pub mod lodestone;
pub mod storage;
pub mod web;
