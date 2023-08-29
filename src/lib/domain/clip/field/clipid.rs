use derive_more::{ From, Constructor };
use serde::{Serialize, Deserialize};
use crate::domain::DbId;

/// The clipid filed for (`Clip`)[crate::domain::clip::Clip]
#[derive(Debug, Clone, From, Constructor, Serialize, Deserialize)]
pub struct ClipId(DbId);

impl ClipId {
    fn into_inner(self) -> DbId {
        self.0
    }
}

impl Default for ClipId {
    fn default() -> Self {
        Self(DbId::nil())
    }
}

