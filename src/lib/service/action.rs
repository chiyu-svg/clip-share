use crate::domain::Clip;
use super::ServiceError;
use super::ask;
use crate::data::DatabasePool;
use crate::data::query;


pub async fn get_clip(
    req: ask::GetClip,
    pool: &DatabasePool
) -> Result<Clip, ServiceError> {
    let user_password = req.password.clone(); //todo
    let clip: Clip = query::get_clip(pool, req).await?.try_into()?;
    if clip.password.has_password() && !user_password.has_password() {
        return Err(ServiceError::PermissionError("Invalid password".to_string()));
    }
    if user_password.has_password() {
        if user_password == clip.password {
            Ok(clip)
        } else {
            Err(ServiceError::PermissionError("Invalid password".to_string()))
        }
    } else {
        Ok(clip)
    }
}

pub async fn new_clip(
    req: ask::NewClip,
    pool: &DatabasePool
) -> Result<Clip, ServiceError> {
    let clip: Clip = query::new_clip(req, pool).await?.try_into()?;
    Ok(clip)
}

pub async fn update_clip(
    req: ask::UpdateClip,
    pool: &DatabasePool
) -> Result<Clip, ServiceError> {
    let clip = query::update_clip(pool, req).await?.try_into()?;
    Ok(clip)
}