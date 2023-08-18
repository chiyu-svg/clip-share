
use super::model;
use super::{ DataError, DatabasePool };
use crate::domain::ShortCode;


type Result<T> = std::result::Result<T, DataError>;

/// get a (`Clip`)[crate::data::model::Clip]
pub async fn get_clip<M: Into<model::GetClip>>(
    pool: &DatabasePool,
    model: M
) -> Result<model::Clip> {
    let model: model::GetClip = model.into();
    let shortcode = model.shortcode;
    Ok(sqlx::query_as!(
        model::Clip,
        "SELECT * FROM clips WHERE shortcode = ?",
        shortcode
    ).fetch_one(pool)
     .await?
    )
}

/// crate a new (`Clip`)[crate::data::model::NewClip]
pub async fn new_clip<M: Into<model::NewClip>>(
    model: M,
    pool: &DatabasePool
) -> Result<model::Clip> {
    let model: model::NewClip = model.into();
    let _ = sqlx::query!(
        r#"INSERT INTO clips (
            clip_id,
            shortcode,
            content,
            title,
            posted,
            expires,
            password,
            hits)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
        model.clip_id,
        model.shortcode,
        model.content,
        model.title,
        model.posted,
        model.expires,
        model.password,
        0
    ).execute(pool)
     .await?;
    get_clip(pool, ShortCode::from(model.shortcode)).await
}

/// Update (`Clip`)[crate::data::model::UpdateClip]
pub async fn update_clip<M: Into<model::UpdateClip>>(
    pool: &DatabasePool,
    model: M
) -> Result<model::Clip> {
    let model: model::UpdateClip = model.into();
    let _ = sqlx::query!(
        r#"UPDATE clips SET
        content = ?,
        expires = ?,
        password = ?,
        title = ?
       WHERE shortcode = ?"#,
        model.content,
        model.expires,
        model.password,
        model.title,
        model.shortcode
    ).execute(pool).await?;
    get_clip(pool, ShortCode::from(model.shortcode)).await
}
