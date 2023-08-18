

use chrono::NaiveDateTime;
use crate::domain::{ ClipError, DbId, Time};
use crate::domain::ShortCode;


/// select and retireved from database
#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: NaiveDateTime,
    pub(in crate::data) expires: Option<NaiveDateTime>,
    pub(in crate::data) password: Option<String>,
    pub(in crate::data) hits: i64
}

impl TryFrom<Clip> for crate::domain::Clip {
    type Error = ClipError;
    fn try_from(clip: Clip) -> Result<Self, Self::Error> {
        use std::str::FromStr;
        use crate::domain::clip::field;
        Ok(
            Self {
                clip_id: field::ClipId::new(DbId::from_str(clip.clip_id.as_str())?),
                shortcode: field::ShortCode::from(clip.shortcode),
                content: field::Content::new(clip.content.as_str())?,
                title: field::Title::new(clip.title),
                posted: field::Posted::new(Time::from_natve_utc(clip.posted)),
                expires: field::Expires::new(clip.expires.map(Time::from_natve_utc)),
                password: field::Password::new(clip.password)?,
                hits: field::Hits::new(u64::try_from(clip.hits)?)
            }
        )
    }
}

/// (`get_clip`)[crate::data::query::get_clip] 查询 [`Clip`]
pub struct GetClip {
    pub(in crate::data) shortcode: String
}

impl GetClip {
    fn new(shortcode: ShortCode) -> Self {
        Self { shortcode: shortcode.into() }
    }
}

impl From<ShortCode> for GetClip {
    fn from(shorcode: ShortCode) -> Self {
        GetClip::new(shorcode)
    }
}

/// (`new_clip`)[crate::data::query::new_clip] 新增 [`Clip`]
pub struct NewClip {
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: NaiveDateTime,
    pub(in crate::data) expires: Option<NaiveDateTime>,
    pub(in crate::data) password: Option<String>
}

/// (`update_clip`)[crate::data::query::update_clip] 根据 shortcode 修改 [`Clip`]
pub struct UpdateClip {
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) expires: Option<NaiveDateTime>,
    pub(in crate::data) password: Option<String>
}