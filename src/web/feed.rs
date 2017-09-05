use jsonfeed;
use rss;

pub type JsonFeedItem = jsonfeed::Item;
pub type RssItem = rss::Item;

pub trait FeedItem {
    fn to_jsonfeed(&self) -> JsonFeedItem;
    fn to_rss(&self) -> RssItem;
}
