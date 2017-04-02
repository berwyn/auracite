use std::result::Result;

use lodestone::NewsItem;
use redis::{Client, Commands, Connection, RedisError};

pub fn connect_redis() -> Connection {
    let client = match Client::open(dotenv!("REDIS_URL")) {
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

    conn.lpush(format!("news:{}", locale), json)
}
