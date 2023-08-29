use rocket::form::{self, FromFormField};
use serde::{Deserialize, Serialize};
use derive_more::From;

use crate::domain::ClipError;

/// The content file for [`Clip`](crate::domain::clip::Clip)
#[derive(Debug, Clone, Serialize, Deserialize, From)]
pub struct Content(String);

impl Content {
    pub fn new(content: &str) -> Result<Self, ClipError> {
        if !content.trim().is_empty() {
            Ok(Content(content.to_owned()))
        } else {
            Err(ClipError::EmptyContent)
        }
    }
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn as_str(&self) -> &str {
        &self.0
    } 
}



#[rocket::async_trait]
impl<'r> FromFormField<'r> for Content {
    fn from_value(field: rocket::form::ValueField<'r>) -> form::Result<'r, Self> {
        Ok(Self::new(field.value).map_err(|err| form::Error::validation(format!("{}", err)))?)
    }
}
