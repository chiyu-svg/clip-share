use crate::domain::ClipError;


/// The content file for [`Clip`](crate::domain::clip::Clip)
#[derive(Debug)]
pub struct Content(String);

impl Content {
    pub fn new(content: &str) -> Result<Self, ClipError> {
        if !content.trim().is_empty() {
            Ok(Content(content.to_owned()))
        } else {
            Err(ClipError::EmptyContent)
        }
    }
}