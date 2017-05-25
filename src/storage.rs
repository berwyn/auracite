use std::result::Result;

use env::load_var;
use lodestone::NewsItem;
use redis::{Client, Commands, Connection, RedisError};

pub fn connect_redis() -> Connection {
    let env = load_var("REDIS_URL", "redis://127.0.0.1:6379");
    let url = env.as_str();
    let client = match Client::open(url) {
        Ok(client) => client,
        Err(err) => panic!("Failed to open redis connection!\n{}", err)
    };

    let conn = match client.get_connection() {
        Ok(conn) => conn,
        Err(err) => panic!("Redis connection could not open!\n{}", err)
    };

    conn
}

pub fn push_news(locale: &'static str, topics: &Vec<NewsItem>, conn: &Connection) -> Result<(), RedisError> {
    let json: Vec<String> = topics.iter().map(|ref t| t.to_json()).collect();

    conn.rpush(format!("news:{}", locale), json)
}

pub fn pull_news(locale: &'static str, conn: &Connection) -> Vec<NewsItem> {
    let vec: Vec<String> = match conn.lrange(format!("news:{}", locale), -20, -1) {
        Ok(vec) => vec,
        Err(err) => panic!("Couldn't get JSON from redis!\n{}", err)
    };

    vec.iter().map(|s| NewsItem::from_json(s)).collect()
}
