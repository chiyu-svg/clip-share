use derive_more::{ From, Constructor };
use crate::domain::DbId;

/// The clipid filed for (`Clip`)[crate::domain::clip::Clip]
#[derive(Debug, From, Constructor)]
pub struct ClipId(DbId);

