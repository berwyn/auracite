use rss::RSSChannelItem;
use serde_json;
use xml::writer::EventWriter;
use xml_ext::write_simple_xml;

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

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl RSSChannelItem for NewsItem {
    fn write_xml(&self, w: &mut EventWriter<&mut Vec<u8>>) {
        write_simple_xml(w, "title", self.title.as_str());
        write_simple_xml(w, "description", self.description.as_str());
        write_simple_xml(w, "link", self.link.as_str());
    }
}
