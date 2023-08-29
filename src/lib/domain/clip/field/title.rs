// todo

use rocket::form::{ self, FromFormField };
use serde::{Serialize, Deserialize};

/// The title for [`Clip`](crate::domain::clip::Clip)
#[derive(Debug, Serialize, Deserialize)]
pub struct Title(Option<String>);

impl Title {
    pub fn new<T: Into<Option<String>>>(title: T) -> Self {
        let title: Option<String> = title.into();
        match title {
            Some(title) => {
                if !title.trim().is_empty() {
                    Title(Some(title))
                } else {
                    Title(None)
                }
            },
            None => Title(None)
        }
    }
    pub fn into_inner(self) -> Option<String> {
        self.0
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Title {
    fn from_value(field: form::ValueField<'r>) -> form::Result<'r,Self> {
        Ok(Self::new(field.value.to_owned()))
    }
}
