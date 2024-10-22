use lazy_static::lazy_static;
use sqlx::{sqlite::SqliteRow, Executor, Row, SqlitePool};
use std::sync::Arc;

pub struct AppState {
    pub db_pool: SqlitePool,
}

lazy_static! {
    pub static ref APP_STATE: Arc<AppState> = {
        let db_url = "sqlite://app.db";  // 指定数据库文件，SQLite会自动创建该文件
        let db_pool = SqlitePool::connect_lazy(db_url).expect("Failed to create DB pool");
        Arc::new(AppState { db_pool })
    };
}

impl AppState {
    // 初始化数据库，自动创建表
    pub async fn init_db(&self) -> Result<(), sqlx::Error> {
        let mut conn = self.db_pool.acquire().await?;
        // 创建 keys 表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS keys (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER,
                key TEXT
            )",
        )
        .await?;
        // 创建 urls 表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS urls (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                key_id INTEGER,
                url TEXT,
                FOREIGN KEY (key_id) REFERENCES keys(id)
            )",
        )
        .await?;

        println!("Database and tables have been initialized.");
        Ok(())
    }

    // 添加 key 到数据库
    pub async fn add_key(&self, uid: i64, key: &str) -> Result<(), sqlx::Error> {
        let mut conn = self.db_pool.acquire().await?;
        // 插入新 key
        sqlx::query("INSERT INTO keys (user_id, key) VALUES (?, ?)")
            .bind(uid)
            .bind(key)
            .execute(&mut *conn)
            .await?;

        Ok(())
    }

    // 检查 key 是否存在
    pub async fn check_key(&self, _uid: Option<i64>, key: &str) -> Result<bool, sqlx::Error> {
        let mut conn = self.db_pool.acquire().await?;
        // 查询全局 key 集合
        let exists_in_global: Option<bool> =
            sqlx::query("SELECT EXISTS(SELECT 1 FROM keys WHERE key = ?)")
                .bind(key)
                .map(|row: SqliteRow| row.get(0))
                .fetch_optional(&mut *conn)
                .await?;

        Ok(exists_in_global.unwrap_or(false))
    }

    // 获取 URL
    pub async fn get_url(&self, key: &str) -> Result<Option<String>, sqlx::Error> {
        let mut conn = self.db_pool.acquire().await?;

        let url: Option<String> =
            sqlx::query("SELECT url FROM urls u JOIN keys k ON u.key_id = k.id WHERE k.key = ?")
                .bind(key)
                .map(|row: SqliteRow| row.get(0))
                .fetch_optional(&mut *conn)
                .await?;

        Ok(url)
    }

    // 添加 URL
    pub async fn add_url(&self, key: &str, url: &str) -> Result<(), sqlx::Error> {
        let mut conn = self.db_pool.acquire().await?;

        // 查询 key_id
        let key_id: Option<i64> = sqlx::query("SELECT id FROM keys WHERE key = ?")
            .bind(key)
            .map(|row: SqliteRow| row.get(0))
            .fetch_optional(&mut *conn)
            .await?;

        let key_id = match key_id {
            Some(id) => id,
            None => return Err(sqlx::Error::RowNotFound), // 如果 key 不存在
        };

        // 插入新 URL
        sqlx::query("INSERT INTO urls (key_id, url) VALUES (?, ?)")
            .bind(key_id)
            .bind(url)
            .execute(&mut *conn)
            .await?;

        Ok(())
    }

    // 删除 URL
    pub async fn delete_url(&self, key: &str, url: &str) -> Result<(), sqlx::Error> {
        let mut conn = self.db_pool.acquire().await?;

        let key_id: Option<i64> = sqlx::query("SELECT id FROM keys WHERE key = ?")
            .bind(key)
            .map(|row: SqliteRow| row.get(0))
            .fetch_optional(&mut *conn)
            .await?;

        if let Some(key_id) = key_id {
            sqlx::query("DELETE FROM urls WHERE key_id = ? AND url = ?")
                .bind(key_id)
                .bind(url)
                .execute(&mut *conn)
                .await?;
        }

        Ok(())
    }

    // 获取所有 URL
    pub async fn get_urls(&self, key: &str) -> Result<Vec<String>, sqlx::Error> {
        let mut conn = self.db_pool.acquire().await?;

        let urls: Vec<String> =
            sqlx::query("SELECT url FROM urls u JOIN keys k ON u.key_id = k.id WHERE k.key = ?")
                .bind(key)
                .map(|row: SqliteRow| row.get(0))
                .fetch_all(&mut *conn)
                .await?;

        Ok(urls)
    }

    // 获取用户的所有 keys
    pub async fn get_user_keys(&self, uid: i64) -> Result<Vec<String>, sqlx::Error> {
        let mut conn = self.db_pool.acquire().await?;

        let keys: Vec<String> = sqlx::query("SELECT k.key FROM keys k WHERE k.user_id = ?")
            .bind(uid)
            .map(|row: SqliteRow| row.get(0))
            .fetch_all(&mut *conn)
            .await?;

        Ok(keys)
    }
}
