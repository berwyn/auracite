use jsonfeed;
use jsonfeed::Feed;
use rss;
use rss::ChannelBuilder;
use storage;

#[get("/rss")]
pub fn rss() -> String {
    let conn = storage::connect_redis().unwrap();
    let items: Vec<rss::Item> = storage::pull_news("na", &conn)
        .unwrap()
        .iter()
        .map(|i| i.to_rss())
        .collect();

    ChannelBuilder::default()
        .title("FINAL FANTASY XIV The Lodestone")
        .link("https://na.finalfantasyxiv.com/lodestone/")
        .items(items)
        .build()
        .unwrap()
        .to_string()
}

#[get("/jsonfeed")]
pub fn jsonfeed() -> String {
    let mut feed = Feed::builder().title("FINAL FANTASY XIV The Lodestone");
    let conn = storage::connect_redis().unwrap();
    let items = storage::pull_news("na", &conn).unwrap();
    for item in items {
        feed = feed.item(item.to_jsonfeed());
    }

    jsonfeed::to_string(&feed.build()).unwrap()
}
