use chrono::{DateTime, Utc, NaiveDateTime};
use derive_more::From;

#[derive(Debug, From)]
pub struct Time(DateTime<Utc>);

impl Time {
    pub fn from_natve_utc(datetime: NaiveDateTime) -> Self {
        Time(DateTime::from_utc(datetime.to_owned(), Utc)) // 其实这里就体现了 to_owned 很好用
    }
}