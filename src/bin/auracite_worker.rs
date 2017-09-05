#![feature(plugin)]
#![plugin(dotenv_macros)]

extern crate auracite;
extern crate dotenv;
extern crate select;
extern crate reqwest;

use std::io::Read;

use auracite::lodestone::NewsItem;
use auracite::storage::{connect_redis, push_news};
use dotenv::dotenv;
use select::document::Document;
use select::predicate::{Class, Name};

const PAGES: [&'static str; 4] = ["na", "jp", "de", "fr"];

fn main() {
    dotenv().ok();
    let conn = connect_redis().unwrap();

    for page in PAGES.into_iter() {
        let url = format!("http://{}.finalfantasyxiv.com/lodestone/topics", page);
        let download = download_page(url.as_str());
        let topics = parse_document(download.as_str());

        match push_news(page, &topics, &conn) {
            Ok(()) => println!("Pushed {} item(s) for locale {}", topics.len(), page),
            Err(err) => panic!("Failed to push lang {}!\n{}", page, err),
        };
    }
}

fn download_page(url: &str) -> String {
    let mut res = reqwest::get(url).unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    body
}

fn parse_document(doc: &str) -> Vec<NewsItem> {
    let mut topics: Vec<NewsItem> = vec![];

    let document = Document::from(doc);
    for (_, topic) in document.find(Class("news__list--topics")).enumerate() {
        let title =
            match topic.find(Class("news__list--title")).into_selection().children().first() {
                Some(node) => node.text(),
                None => "".to_string(),
            };

        let banner = topic.find(Class("news__list--banner")).into_selection();
        let mut body: String = String::from("");
        for para in banner.find(Name("p")).iter() {
            if para.text().len() > 0 {
                body = para.text();
            }
        }

        if body.len() == 0 {
            panic!("Couldn't find description!");
        }

        let link = match banner.find(Class("news__list--img")).first() {
            Some(node) => node.attr("href").unwrap().to_string(),
            None => "".to_string(),
        };

        let item = NewsItem::create(title, body, link);
        topics.push(item);
    }

    topics
}
