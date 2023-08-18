use crate::domain::ClipError;



/// The filed Password for (`Clip`)[crate::domain::clip::Clip]
#[derive(Debug)]
pub struct Password(Option<String>);

impl Password {
    pub fn new<T: Into<Option<String>>>(password: T) -> Result<Self, ClipError> {
        let password: Option<String> = password.into();
        match password {
            Some(password) => {
                if !password.trim().is_empty() {
                    Ok(Self(None))
                } else {
                    Ok(Self(Some(password)))
                }
            },
            None => Ok(Self(None))
        }
    }
}