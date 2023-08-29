use std::str::FromStr;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use derive_more::From;


// 数据库内部 Id, 任何用作数据库内部的 id 都会使用这个字段
#[derive(Debug, Clone, From, Serialize, Deserialize)]
pub struct DbId(Uuid);


impl DbId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    pub fn nil() -> DbId {
        Self(Uuid::nil())
    }
}

impl FromStr for DbId {
    type Err = uuid::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
       Ok(DbId(Uuid::parse_str(id)?))
    }
}

impl From<DbId> for String {
    fn from(db_id: DbId) -> Self {
        format!("{}", db_id.0)
    }
}

impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}
