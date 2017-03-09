#![feature(plugin)]
#![plugin(maud_macros)]
#![plugin(rocket_codegen)]

extern crate maud;
extern crate rocket;
extern crate xml;

mod html;

pub mod lodestone;
pub mod rss;
pub mod web;
