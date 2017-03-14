extern crate auracite;
extern crate hyper;
extern crate select;

use std::io::{Read};

use auracite::lodestone::{NewsItem};
use hyper::{Client};
use select::document::Document;
use select::predicate::{Class, Name};

const PAGES: [&'static str; 4] = [
    "na", "jp", "de", "fr"
];

fn main() {
    for page in PAGES.into_iter() {
        let url = format!("http://{}.finalfantasyxiv.com/lodestone/topics", page);
        let download = download_page(url.as_str());
        let topics = parse_document(download.as_str());
        println!("{} -- {}", page, topics.len());
    }
}

fn download_page(url: &str) -> String {
    let client = Client::new();
    let mut res = client.get(url).send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    body
}

fn parse_document(doc: &str) -> Vec<NewsItem> {
    let mut topics: Vec<NewsItem> = vec![];

    let document = Document::from(doc);
    let items = document.find(Class("news__content__list__topics")).children();
    for topic in items.filter(Name("li")).iter() {
        let title = match topic.find(Class("ic_topics")).children().first() {
            Some(node) => node.text(),
            None => "".to_string(),
        };

        let body = match topic.find(Class("news__content__list__topics--body")).first() {
            Some(node) => node.text(),
            None => "".to_string(),
        };

        let link = match topic.find(Class("news__content__list__topics__link_banner")).first() {
            Some(node) => node.attr("href").unwrap().to_string(),
            None => "".to_string(),
        };

        let item = NewsItem::create(title, body, link);
        topics.push(item);
    }

    topics
}
