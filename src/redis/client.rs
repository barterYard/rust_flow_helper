use std::env;

pub struct Redis {
    pub client: redis::Client,
}
impl Redis {
    pub fn new() -> Self {
        let redis_url = env::var("REDIS_URL").expect("redis url (REDIS_URL) not set"); //redis://127.0.0.1:6379
        Redis {
            client: redis::Client::open(redis_url).unwrap(),
        }
    }
    pub fn connection(&self) -> redis::Connection {
        self.client.get_connection().unwrap()
    }
}
