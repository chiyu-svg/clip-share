use derive_more::Constructor;


/// The field Hits for (`Clip`)[crate::domain::clip::Clip]
#[derive(Debug, Constructor)]
pub struct Hits(u64);