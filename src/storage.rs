use env::load_var;
use lodestone::NewsItem;
use web::feed::FeedItem;
use redis::{Client, Commands, Connection, RedisError};

const NEWS_KEY: &'static str = "auracite:news";

pub fn connect_redis() -> Result<Connection, RedisError> {
    let env = load_var("REDIS_URL", "redis://127.0.0.1:6379");
    let url = env.as_str();
    let client = Client::open(url)?;
    let conn = client.get_connection()?;
    Ok(conn)
}

pub fn push_news(locale: &'static str,
                 topics: &Vec<NewsItem>,
                 conn: &Connection)
                 -> Result<(), RedisError> {
    let json: Vec<String> = topics.iter().map(|ref t| t.to_json()).collect();

    conn.rpush(format!("{}:{}", NEWS_KEY, locale), json)
}

pub fn pull_news(locale: &String,
                 conn: &Connection)
                 -> Result<Vec<Box<FeedItem>>, RedisError> {
    let vec: Vec<String> = conn.lrange(format!("{}:{}", NEWS_KEY, locale), -20, -1)?;

    Ok(vec.iter().map(|s| Box::new(NewsItem::from_json(s)) as Box<FeedItem>).collect())
}
