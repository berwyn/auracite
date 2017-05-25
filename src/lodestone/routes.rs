use jsonfeed::{JSONFeed, JSONFeedConvertable, JSONFeedItem};
use rss::{RSS, RSSChannel, RSSChannelItem};
use storage::{connect_redis, pull_news};

#[get("/")]
pub fn index() -> &'static str {
    "Lodestone root"
}

#[get("/rss")]
pub fn rss() -> RSS {
    let items = pull_news("na", &connect_redis()).into_iter().map(box_news).collect();

    RSS {
        channel: RSSChannel {
            title: String::from("FINAL FANTASY XIV, The Lodestone"),
            description: String::from("Official community site for FINAL FANTASY XIV: A Realm Reborn."),
            link: String::from("http://na.finalfantasyxiv.com/lodestone/"),
            ttl: 1800,
            items: items
        }
    }
}

#[get("/jsonfeed")]
pub fn jsonfeed() -> JSONFeed {
    let items = pull_news("na", &connect_redis()).into_iter().map(convert_news).collect();

    let mut feed = JSONFeed::new(
        1,
        String::from("FINAL FANTASY XIV, The Lodestone"),
        String::from("Official community site for FINAL FANTASY XIV: A Realm Reborn"),
        String::from("http://na.finalfantasyxiv.com/lodestone/"));
    feed.items = items;

    feed
}

fn box_news<T: RSSChannelItem + 'static>(item: T) -> Box<RSSChannelItem> {
    Box::new(item)
}

fn convert_news<T: JSONFeedConvertable + 'static>(item: T) -> JSONFeedItem {
    item.convert()
}
