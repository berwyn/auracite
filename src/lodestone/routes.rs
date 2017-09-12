use jsonfeed;
use jsonfeed::Feed;
use rocket::http::RawStr;
use rss;
use rss::ChannelBuilder;
use storage;

fn lang_accepted(lang: &String) -> bool {
    lang == "na"
        || lang == "de"
        || lang == "jp"
        || lang == "fr"
}

#[get("/<lang>/rss")]
pub fn rss(lang: &RawStr) -> Option<String> {
    let parsed = lang.to_lowercase();
    if !lang_accepted(&parsed) {
        return None;
    }

    let conn = storage::connect_redis().unwrap();
    let items: Vec<rss::Item> = storage::pull_news(&lang.to_lowercase(), &conn)
        .unwrap()
        .iter()
        .map(|i| i.to_rss())
        .collect();

    let content = ChannelBuilder::default()
        .title("FINAL FANTASY XIV The Lodestone")
        .link("https://na.finalfantasyxiv.com/lodestone/")
        .items(items)
        .build()
        .unwrap()
        .to_string();
    
    Some(content)
}

#[get("/<lang>/jsonfeed")]
pub fn jsonfeed(lang: &RawStr) -> Option<String> {
    let parsed = lang.to_lowercase();
    if !lang_accepted(&parsed) {
        return None;
    }

    let mut feed = Feed::builder().title("FINAL FANTASY XIV The Lodestone");
    let conn = storage::connect_redis().unwrap();
    let items = storage::pull_news(&lang.to_lowercase(), &conn).unwrap();
    for item in items {
        feed = feed.item(item.to_jsonfeed());
    }

    let content = jsonfeed::to_string(&feed.build()).unwrap();

    Some(content)
}
