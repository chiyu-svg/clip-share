pub mod model;
pub mod query;
use sqlx::{Sqlite, ConnectOptions};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("database get error: {0}")]
    DataBase(#[from] sqlx::Error)
}

// 数据库连接池包装器
pub type AppDataBase = Database<Sqlite>;
// 连接池
pub type DatabasePool = sqlx::sqlite::SqlitePool;

// 这里的 D 是泛型约束，并不是什么其它的东西
pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

impl Database<Sqlite> {
    // 创建连接池
    pub async fn new(connection_str: &str) -> Self {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(connection_str)
            .await;
        match pool {
            Ok(pool) => Self(pool),
            Err(_) => {
                eprintln!("please run $ sqlx database up");
                panic!("database connection failed")
            }
        }
    }
    /// 获取数据库连接池[`DatabasePool`]
    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }
}