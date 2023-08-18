use std::str::FromStr;

use uuid::Uuid;


// 数据库内部 Id, 任何用作数据库内部的 id 都会使用这个字段
#[derive(Debug)]
pub struct DbId(Uuid);

impl DbId {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl FromStr for DbId {
    type Err = uuid::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
       Ok(DbId(Uuid::parse_str(id)?))
    }
}