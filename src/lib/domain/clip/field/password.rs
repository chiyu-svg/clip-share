use std::str::FromStr;

use rocket::form::{ self, FromFormField };
use serde::{Serialize, Deserialize};
use derive_more::From;
use crate::domain::ClipError;



/// The filed Password for (`Clip`)[crate::domain::clip::Clip]
#[derive(Debug, Clone, From,  PartialEq, Eq, Serialize, Deserialize)]
pub struct Password(Option<String>);

impl Password {
    pub fn new<T: Into<Option<String>>>(password: T) -> Result<Self, ClipError> {
        let password: Option<String> = password.into();
        match password {
            Some(password) => {
                if !password.trim().is_empty() {
                    Ok(Self(Some(password)))
                } else {
                    Ok(Self(None))
                }
            },
            None => Ok(Self(None))
        }
    }
    pub fn has_password(&self) -> bool {
        self.0.is_some()
    }
    pub fn into_inner(self) -> Option<String> {
        self.0
    }
}
impl Default for Password {
    fn default() -> Self {
        Self(None)
    }
}
impl FromStr for Password {
    type Err = ClipError;
    fn from_str(password: &str) -> Result<Self, Self::Err> {
        Self::new(password.to_string())
    }
}

#[rocket::async_trait]
impl<'t> FromFormField<'t> for Password {
    fn from_value(field: form::ValueField<'t>) -> form::Result<'t,Self> {
        Ok(Self::new(field.value.to_owned()).map_err(|e| form::Error::validation(format!("{}", e)))?)
    }
}