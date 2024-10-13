use lazy_static::lazy_static;
use oauth2::basic::BasicClient;
use redis::{self, AsyncCommands};
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{error::AppError, oauth2_client};

const REDIS_PREFIX: &str = "URL_BALANCING:";
const REDIS_KEY: &str = "KEY";
const REDIS_LIST_PREFIX: &str = "LIST:";
const REDIS_CSRF: &str = "CSRF";
pub struct AppState {
    pub redis_client: redis::Client,
    pub oauth2_client: BasicClient,
}

lazy_static! {
    pub static ref APP_STATE: Arc<AppState> = {
        let redis_url =
            std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
        let redis_password = std::env::var("REDIS_PASSWORD").unwrap_or_else(|_| "".to_string());

        let redis_url_with_password = if redis_password.is_empty() {
            redis_url
        } else {
            let url = url::Url::parse(&redis_url).expect("Invalid Redis URL");
            let mut url_with_password = url.clone();
            url_with_password
                .set_password(Some(&redis_password))
                .expect("Failed to set password");
            url_with_password.to_string()
        };

        let redis_client =
            redis::Client::open(redis_url_with_password).expect("Invalid Redis client");
        let oauth2_client = oauth2_client().expect("Invalid OAuth2 client");
        Arc::new(AppState {
            redis_client,
            oauth2_client,
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
    pub async fn add_key(&self, key: &str) -> Result<(), AppError> {
        let key_set = concat_string!(REDIS_PREFIX, REDIS_KEY);
        let mut con = self.redis_client.get_multiplexed_tokio_connection().await?;
        Ok(con.sadd(key_set, key).await?)
    }

    pub async fn check_key(&self, key: &str) -> Result<bool, AppError> {
        let key_set = format!("{}{}", REDIS_PREFIX, REDIS_KEY);
        let mut con = self.redis_client.get_multiplexed_tokio_connection().await?;
        Ok(con.sismember(key_set, key).await?)
    }

    pub async fn get_url(&self, key: &str) -> Result<String, AppError> {
        let key = concat_string!(REDIS_PREFIX, REDIS_LIST_PREFIX, key);
        let mut con = self.redis_client.get_multiplexed_tokio_connection().await?;
        Ok(con.srandmember(key).await?)
    }

    pub async fn add_url(&self, key: &str, url: &str) -> Result<(), AppError> {
        let key = concat_string!(REDIS_PREFIX, REDIS_LIST_PREFIX, key);
        let mut con = self.redis_client.get_multiplexed_tokio_connection().await?;
        Ok(con.sadd(&key, url).await?)
    }

    pub async fn delete_url(&self, key: &str, url: &str) -> Result<(), AppError> {
        let key = concat_string!(REDIS_PREFIX, REDIS_LIST_PREFIX, key);
        let mut con = self.redis_client.get_multiplexed_tokio_connection().await?;
        Ok(con.srem(&key, url).await?)
    }

    pub async fn get_urls(&self, key: &str) -> Result<Vec<String>, AppError> {
        let key = concat_string!(REDIS_PREFIX, REDIS_LIST_PREFIX, key);
        let mut con = self.redis_client.get_multiplexed_tokio_connection().await?;
        Ok(con.smembers(key).await?)
    }

    pub async fn set_csrf(&self, csrf: &str) -> Result<(), AppError> {
        let key = concat_string!(REDIS_PREFIX, REDIS_CSRF);
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut con = self.redis_client.get_multiplexed_tokio_connection().await?;
        let _: () = con.zadd(&key, csrf, current_time + 10 * 60).await?;
        Ok(con.zrembyscore(&key, "-inf", current_time).await?)
    }
    pub async fn check_csrf(&self, csrf: &str) -> Result<bool, AppError> {
        let key = concat_string!(REDIS_PREFIX, REDIS_CSRF);
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut con = self.redis_client.get_multiplexed_tokio_connection().await?;
        let _: () = con.zrembyscore(&key, "-inf", current_time).await?;
        let rank: Option<isize> = con.zrank(key, csrf).await?;
        Ok(rank.is_some())
    }
}
