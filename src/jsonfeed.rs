use std::io::Cursor;
use rocket::http::{ContentType, Status};
use rocket::response::{Responder, Response};
use serde_json::to_string;

pub trait JSONFeedConvertable {
    fn convert(self) -> JSONFeedItem;
}

/// A trait for author information
#[derive(Serialize)]
pub struct JSONFeedAuthor {
    pub name: String,
    pub url: String,
    pub avatar: String,
}

/// A trait allowing arbitrary items to be serialised into a feed
#[derive(Serialize)]
pub struct JSONFeedItem {
    pub id: String,
    pub title: String,
    pub url: String,
    pub external_url: Option<String>,
    pub content_html: Option<String>,
    pub content_text: Option<String>,
    pub author: Option<JSONFeedAuthor>,
    pub date_published: Option<String>,
}

/// A feed of news items per the JSONFeed spec
#[derive(Serialize)]
pub struct JSONFeed {
    /// The name of the feed or site
    pub title: String,
    /// A short tagline describing the feed/site
    pub description: String,
    /// The home page URL of the site
    pub home_page_url: String,
    /// An optional icon to show for the feed
    pub icon: Option<String>,
    /// A list of items to show in the feed
    pub items: Vec<JSONFeedItem>,
}

impl<'r> Responder<'r> for JSONFeed {
    fn respond(self) -> Result<Response<'r>, Status> {
        Response::build()
            .header(ContentType::JSON)
            .sized_body(Cursor::new(to_string(&self).unwrap()))
            .ok()
    }
}
