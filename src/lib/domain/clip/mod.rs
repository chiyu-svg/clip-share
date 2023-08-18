pub mod field;
use thiserror::Error;

/// ClipError 会将在创建每个字段发成的错误类型转换成该错误累心
#[derive(Debug, Error)]
pub enum ClipError {
    #[error("invalid title: {0}")]
    InvalidTitle(String),
    #[error("empty content")]
    EmptyContent,
    #[error("invalid password: {0}")]
    InvalidPassword(String),
    /// (`DbId`)[crate::domain::DbId] parse faile
    #[error("id parse error: {0}")]
    DbId(#[from] uuid::Error),
    #[error("hits parse error: {0}")]
    Hits(#[from] std::num::TryFromIntError)
}



/// Clip 结构体包含 clip 的所有内容信息
/// 每个字段都使用 newType 类型，任何字段创建错误都无法创建出 Clip，从而保证了 Clip 不管在哪里使用都是有效的
/// 而且当我想修改某个字段的时候，只需要修改对应的内部字段就好了， 不需要海东其他地方
pub struct Clip {
    pub clip_id: field::ClipId,
    // 系统会为每一个 clip 创建一个 short_code, 用来定位对应的Clip
    pub shortcode: field::ShortCode,
    pub title: field::Title,
    pub posted: field::Posted,
    pub password: field::Password,
    pub hits: field::Hits,
    pub content: field::Content,
    pub expires: field::Expires
    
}
