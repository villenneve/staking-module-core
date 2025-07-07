use redis::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct RedisClient {
    pub client: Arc<Client>,
}

impl RedisClient {
    pub async fn new() -> Self {
        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL not set");
        let client = redis::Client::open(redis_url).expect("Erro ao conectar ao Redis");
        RedisClient {
            client: Arc::new(client),
        }
    }
}
