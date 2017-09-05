use crypto::digest::Digest;
use crypto::sha1::Sha1;
use jsonfeed;
use rss;
use serde_json;
use web::feed::{FeedItem, JsonFeedItem, RssItem};

/// An RSS Feed item representing a news topic as Lodestone displays it.
#[derive(Serialize, Deserialize, Debug)]
pub struct NewsItem {
    pub title: String,
    pub description: String,
    pub link: String,
}

impl NewsItem {
    /// Constructs a new `NewsItem`
    ///
    /// These items will be represented verbatim in the resulting XMLRSS, so they should have at
    /// least an empty string
    ///
    /// # Examples
    ///
    /// ```rust
    /// let title = String::from("Cool news!");
    /// let description = String::from("There's some really cool news you should check out!");
    /// let link = String::from("https://www.coolsite.com/news-story");
    /// let item = auracite::lodestone::NewsItem::create(title, description, link);
    /// ```
    pub fn create(title: String, description: String, link: String) -> NewsItem {
        NewsItem {
            title: title,
            description: description,
            link: link,
        }
    }

    /// Constructs a `NewsItem` using a JSON string as the source
    ///
    /// # Examples
    ///
    /// ```rs
    /// let json = String::from("{ title:\"Breaking News!\", ... ");
    /// let item = NewsItem::from_json(json);
    /// ```
    pub fn from_json(json: &String) -> NewsItem {
        serde_json::from_str(json.as_str()).unwrap()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl FeedItem for NewsItem {
    fn to_jsonfeed(&self) -> JsonFeedItem {
        let mut item = jsonfeed::Item::builder()
            .title(self.title.clone())
            .content_html(self.description.clone());

        let mut id = Sha1::new();
        id.input(self.title.as_bytes());
        item.id = Some(id.result_str());
        item.build()
            .unwrap()
    }

    fn to_rss(&self) -> RssItem {
        rss::ItemBuilder::default()
            .title(self.title.clone())
            .description(self.description.clone())
            .content(self.description.clone())
            .build()
            .unwrap()
    }
}
