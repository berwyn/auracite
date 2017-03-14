use rss::{RSS, RSSChannel};
use lodestone::{NewsItem};

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
