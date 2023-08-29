use std::str::FromStr;
use rocket::form::{ self, FromFormField };
use serde::{Serialize, Deserialize};
use crate::domain::ClipError;
use crate::domain::Time;
use derive_more::From;

/// The file expires for (`Clip`)[crate::domain::clip::Clip]
#[derive(Debug, Clone, Serialize, Deserialize, From)]
pub struct Expires(Option<Time>);

impl Expires {
    pub fn new<T: Into<Option<Time>>>(time: T) -> Self {
        let time: Option<Time> = time.into();
        match time {
            Some(time) => Expires(Some(time)),
            None => Expires(None)
        }
    }
    pub fn into_inner(self) -> Option<Time> {
        self.0
    }
}
impl Default for Expires {
    fn default() -> Self {
        Self(None)
    }
}


impl FromStr for Expires {
    type Err = ClipError;
    fn from_str(time: &str) -> Result<Self, Self::Err> {
        if time.trim().is_empty() {
            Ok(Self(None))
        } else {
            match Time::from_str(time) {
                Ok(time) => Ok(Self::new(time)),
                Err(err) => Err(err.into())
            }
        }
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Expires {
    fn from_value(field: form::ValueField<'r>) -> form::Result<'r,Self> {
        if field.value.is_empty() {
            Ok(Self(None))
        } else {
            Ok(Self::from_str(field.value).map_err(|e| form::Error::validation(format!("{}", e)))?)
        }
    }
}