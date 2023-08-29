use std::str::FromStr;

use derive_more::From;
use rocket::form::{ self, FromFormField };
use rocket::request::FromParam;
use rocket::{ UriDisplayPath, UriDisplayQuery };
use serde::{Serialize, Deserialize};

/// the shortcode field for (`Clip`)[crate::domain::clip::Clip]
#[derive(Debug, Clone, From, Serialize, Deserialize, UriDisplayPath, UriDisplayQuery)]
pub struct ShortCode(String);

impl ShortCode {
    pub fn new() -> Self {
        use rand::prelude::*;
        let mut shortcode = String::with_capacity(10);
        let allowed_chars = ['b', 'c', 'd', 'e', 'f', 'g', '1', '2', '3', '5'];
        let mut rng = thread_rng();
        for _ in 0..10 {
            shortcode.push(
                *allowed_chars
                    .choose(&mut rng)
                    .expect("sampling file")
            );
        };
        Self(shortcode)
    }
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<ShortCode> for String {
    fn from(shortcode: ShortCode) -> Self {
        shortcode.0.clone()
    }
}

impl<'a> FromParam<'a> for ShortCode {
    type Error = &'a str;
    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        Ok(Self(param.to_owned()))
    }
}

impl From<&str> for ShortCode {
    fn from(shortcode: &str) -> Self {
        Self(shortcode.to_owned())
    }
}

#[rocket::async_trait]
impl<'v> FromFormField<'v> for ShortCode {
    fn from_value(field: form::ValueField<'v>) -> form::Result<'v,Self> {
        Ok(field.value.into())
    }
}
