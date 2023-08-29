use crate::domain::field;
use crate::domain::ShortCode;
use crate::domain::field::Password;
use crate::web::form;

/// The (`GetClip`) fro [`crate::service::action::get_clip`]
pub struct GetClip {
    pub shortcode: field::ShortCode,
    pub password: field::Password
}

impl From<ShortCode> for GetClip {
    fn from(shortcode: ShortCode) -> Self {
        Self { 
            shortcode, 
            password: Password::default() 
        }
    }
}

/// The (`NewClip`) fro [`crate::service::action::new_clip`]
pub struct NewClip {
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password
}

impl From<form::NewClip> for NewClip {
    fn from(clip: form::NewClip) -> Self {
        Self { 
                content: clip.content,
                title: clip.title, 
                expires: clip.expires, 
                password: clip.password 
            }
    }
}

impl TryFrom<form::NewClipJson> for NewClip {
    type Error = crate::web::PageError;
    fn try_from(clip: form::NewClipJson) -> Result<Self, Self::Error> {
        let expires_temp: Result<field::Expires, Self::Error> = clip.expires.try_into();
        match expires_temp {
            Ok(expires) => Ok(Self { 
                content: clip.content, 
                title: clip.title, 
                expires, 
                password: clip.password
            }),
            Err(e) => Err(e)
        }
    }
}

/// The(`UpdateClip`) from [`crate::service::action::update_clip`]
pub struct UpdateClip {
    pub shortcode: field::ShortCode,
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password
}

impl From<form::UpdateClip> for UpdateClip {
    fn from(clip: form::UpdateClip) -> Self {
        Self {
            shortcode: clip.shortcode,
            content: clip.content,
            title: clip.title,
            expires: clip.expires,
            password: clip.password
        }
    }
}

impl TryFrom<form::UpdateClipJson> for UpdateClip {
    type Error = crate::web::PageError;
    fn try_from(clip: form::UpdateClipJson) -> Result<Self, Self::Error> {
        let expires_time: field::Expires = clip.expires.try_into()?;
        Ok(Self {
            shortcode: clip.shortcode,
            content: clip.content,
            title: clip.title,
            expires: expires_time,
            password: clip.password
        })
    }
}


