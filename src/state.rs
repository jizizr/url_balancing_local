use lazy_static::lazy_static;
use redis::{self, AsyncCommands};
use std::sync::Arc;

const REDIS_PREFIX: &str = "URL_BALANCING:";
const REDIS_KEY: &str = "KEY";
const REDIS_LIST_PREFIX: &str = "LIST:";
pub struct AppState {
    pub redis_client: redis::Client,
}

lazy_static! {
    pub static ref APP_STATE: Arc<AppState> = {
        // 初始化 Redis 客户端
        let redis_client = redis::Client::open("redis://127.0.0.1/").expect("无法连接到 Redis");

        Arc::new(AppState {
            redis_client,
        })
    };
}

macro_rules! concat_string {
    // 匹配多个参数，并在最终的表达式中拼接它们
    ($($arg:expr),*) => {{
        // 计算所需的总长度
        let capacity = 0 $(+ $arg.len())*;

        // 提前分配容量
        let mut s = String::with_capacity(capacity);

        // 将所有参数逐一拼接
        $(s.push_str($arg);)*

        s
    }};
}

impl AppState {
    pub async fn add_key(&self, key: &str) -> Result<(), ()> {
        let key_set = concat_string!(REDIS_PREFIX, REDIS_KEY);
        let mut con = self
            .redis_client
            .get_multiplexed_tokio_connection()
            .await
            .map_err(|_| ())?;
        con.sadd(key_set, key).await.map_err(|_| ())?;
        Ok(())
    }

    pub async fn check_key(&self, key: &str) -> Result<bool, ()> {
        let key_set = format!("{}{}", REDIS_PREFIX, REDIS_KEY);
        let mut con = self
            .redis_client
            .get_multiplexed_tokio_connection()
            .await
            .map_err(|_| ())?;
        Ok(con.sismember(key_set, key).await.map_err(|_| ())?)
    }

    pub async fn get_url(&self, key: &str) -> Result<String, ()> {
        let key = concat_string!(REDIS_PREFIX, REDIS_LIST_PREFIX, key);
        let mut con = self
            .redis_client
            .get_multiplexed_tokio_connection()
            .await
            .map_err(|_| ())?;
        Ok(con.srandmember(key).await.map_err(|_| ())?)
    }

    pub async fn add_url(&self, key: &str, url: &str) -> Result<(), ()> {
        let key = concat_string!(REDIS_PREFIX, REDIS_LIST_PREFIX, key);
        let mut con = self
            .redis_client
            .get_multiplexed_tokio_connection()
            .await
            .map_err(|_| ())?;
        con.sadd(&key, url).await.map_err(|_| ())?;
        Ok(())
    }

    pub async fn delete_url(&self, key: &str, url: &str) -> Result<(), ()> {
        let key = concat_string!(REDIS_PREFIX, REDIS_LIST_PREFIX, key);
        let mut con = self
            .redis_client
            .get_multiplexed_tokio_connection()
            .await
            .map_err(|_| ())?;
        con.srem(&key, url).await.map_err(|_| ())?;
        Ok(())
    }

    pub async fn get_urls(&self, key: &str) -> Result<Vec<String>, ()> {
        let key = concat_string!(REDIS_PREFIX, REDIS_LIST_PREFIX, key);
        let mut con = self
            .redis_client
            .get_multiplexed_tokio_connection()
            .await
            .map_err(|_| ())?;
        Ok(con.smembers(key).await.map_err(|_| ())?)
    }
}
