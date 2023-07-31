use std::env;

use redis::{RedisResult, Client};

pub fn connect() -> RedisResult<Client> {
    let url = env::var("REDIS_DATABASE_URL");
    redis::Client::open(url)
}