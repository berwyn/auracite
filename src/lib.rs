#![feature(plugin)]
#![plugin(dotenv_macros)]
#![plugin(maud_macros)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

extern crate dotenv;
extern crate maud;
extern crate redis;
extern crate rocket;
extern crate serde_json;
extern crate xml;

mod html;
mod xml_ext;

pub mod env;
pub mod lodestone;
pub mod storage;
pub mod rss;
pub mod web;
