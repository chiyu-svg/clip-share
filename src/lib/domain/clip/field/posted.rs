use crate::domain::Time;
use derive_more::Constructor;


/// The posted field for (`Clip`)[crate::domain::clip::Clip]
#[derive(Debug, Constructor)]
pub struct Posted(Time);