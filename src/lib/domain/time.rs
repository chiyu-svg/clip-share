use std::str::FromStr;

use chrono::{DateTime, Utc, NaiveDateTime};
use derive_more::From;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, From, Serialize, Deserialize)]
pub struct Time(DateTime<Utc>);

impl Time {
    pub fn from_natve_utc(datetime: NaiveDateTime) -> Self {
        Time(DateTime::from_utc(datetime.to_owned(), Utc)) // 其实这里就体现了 to_owned 很好用
    }
    pub fn timestamp(self) -> i64 {
        self.0.timestamp()
    }
    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }
}

impl FromStr for Time {
    type Err = chrono::ParseError;
    fn from_str(time: &str) -> Result<Self, Self::Err> {
        match format!("{}T00:00:00Z", time).parse::<DateTime<Utc>>() {
            Ok(time) => Ok(Self(time.into())),
            Err(e) => Err(e)
        }
    }
}
#[cfg(test)]
mod test {
    use chrono::NaiveDate;
    use super::*;
    #[test]
    fn modify_test() {
        let a = NaiveDate::from_ymd_opt(2012, 10, 20).unwrap().and_hms_opt(10, 30, 36).unwrap();
        let b = Time::from_natve_utc(a);
        let c = serde_json::to_string(&b).unwrap();
        let d = serde_json::from_str::<Time>(&c);
        println!("c{}", c);
        println!("d{:?}", d);
    }
}