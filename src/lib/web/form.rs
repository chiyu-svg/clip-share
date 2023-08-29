use rocket::{form::FromForm, request::{FromRequest, self}, Request};
use rocket::request::Outcome;
use serde::{Deserialize, Serialize};
use crate::domain::field;
use super::PageError;
use std::str::FromStr;

#[derive(Debug, FromForm, Deserialize, Serialize)]
pub struct NewClip {
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequestTime(Option<String>);

impl TryFrom<RequestTime> for field::Expires {
    type Error = PageError;
    fn try_from(request_time: RequestTime) -> Result<Self, Self::Error> {
        match request_time.0 {
            Some(time) => match Self::from_str(&time) {
                Ok(time_inner) => Ok(time_inner),
                Err(_) => Err(PageError::ParseError("time field parse faild".to_string()))
            },
            None => Ok(Self::new(None))
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewClipJson {
    pub content: field::Content,
    pub title: field::Title,
    pub expires: RequestTime,
    pub password: field::Password
}


#[derive(Debug, FromForm)]
pub struct UpdateClip {
    pub shortcode: field::ShortCode,
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateClipJson {
    pub shortcode: field::ShortCode,
    pub content: field::Content,
    pub title: field::Title,
    pub expires: RequestTime,
    pub password: field::Password
}
