pub mod ask;
pub mod action;

use thiserror::Error;
use crate::domain::ClipError;
use crate::data::DataError;


#[derive(Debug,Error)]
pub enum ServiceError {
    #[error("clip error: {0}")]
    Clip(#[from] ClipError),
    #[error("database error: {0}")]
    Data(DataError),
    #[error("data not found")]
    NotFound,
    #[error("password not met: {0}")]
    PermissionError(String)
}

impl From<DataError> for ServiceError {
    fn from(error: DataError) -> Self {
        match error {
            DataError::DataBase(e) => {
                match e {
                    sqlx::Error::RowNotFound => Self::NotFound,
                    other => Self::Data(DataError::DataBase(other))
                }
            }
        }
    }
}