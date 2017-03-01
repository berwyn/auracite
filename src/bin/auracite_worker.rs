extern crate hyper;
extern crate select;

use std::io::{Read};

use hyper::{Client};
use select::document::Document;
use select::predicate::{Class};

fn main() {
    println!("Hello, Worker!");
    let homepage = download_page("http://na.finalfantasyxiv.com/lodestone/topics/");
    parse_document(homepage.as_str());
}

fn download_page(url: &str) -> String {
    let client = Client::new();
    let mut res = client.get(url).send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    body
}

fn parse_document(doc: &str) {
    let document = Document::from(doc);
    for topic in document.find(Class("news__content__list__topics")).children().iter() {
        let title = topic.find(Class("ic_topics")).children().first();
        match title {
            Some(node) => println!("{:?}", node.text()),
            None => {},
        }
    }
}
