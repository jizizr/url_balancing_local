use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use redis;
use lazy_static::lazy_static;

pub struct AppState {
    pub redis_client: redis::Client,
    pub key_map: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

lazy_static! {
    pub static ref APP_STATE: Arc<AppState> = {
        // 初始化 Redis 客户端
        let redis_client = redis::Client::open("redis://127.0.0.1/").expect("无法连接到 Redis");
        let key_map = Arc::new(Mutex::new(HashMap::new()));

        Arc::new(AppState {
            redis_client,
            key_map,
        })
    };
}