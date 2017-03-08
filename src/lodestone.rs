use rss::{RSS, RSSChannel, RSSChannelItem, write_simple_xml};
use xml::writer::EventWriter;

#[derive(Debug)]
pub struct NewsItem {
    title: String,
    description: String,
    link: String,
}

impl NewsItem {
    pub fn create(title: String, description: String, link: String) -> NewsItem {
        NewsItem {
            title: title,
            description: description,
            link: link,
        }
    }
}

impl RSSChannelItem for NewsItem {
    fn write_xml(&self, w: &mut EventWriter<&mut Vec<u8>>) {
        write_simple_xml(w, "title", self.title.as_str());
        write_simple_xml(w, "description", self.description.as_str());
        write_simple_xml(w, "link", self.link.as_str());
    }
}

#[get("/")]
pub fn index() -> &'static str {
    "Lodestone root"
}

#[get("/rss")]
pub fn rss() -> RSS {
    RSS {
        channel: RSSChannel {
            title: String::from("FINAL FANTASY XIV, The Lodestone"),
            description: String::from("Official community site for FINAL FANTASY XIV: A Realm Reborn."),
            link: String::from("http://na.finalfantasyxiv.com/lodestone/"),
            ttl: 1800,
            items: vec![
                Box::from(
                    NewsItem {
                        title: String::from("Little Ladies' Day"),
                        description: String::from("As the streets are painted in the sweet pink of spring and the fragrant scent of cherry blossoms lure men, women, and children from their homes, it is clear Little Ladies’ Day is upon Eorzea, once more."),
                        link: String::from("http://na.finalfantasyxiv.com/lodestone/topics/detail/ef9c1c207b60fb46060f8e908ee668e2d7e4e72f"),
                    }
                )
            ]
        }
    }
}