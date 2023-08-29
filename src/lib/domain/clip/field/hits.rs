use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use derive_more::From;


/// The field Hits for (`Clip`)[crate::domain::clip::Clip]
#[derive(Debug, Clone, From, Constructor, Serialize, Deserialize)]
pub struct Hits(u64);

impl Hits {
    fn into_inner(self) -> u64 {
        self.0
    }
}

impl Default for Hits {
    fn default() -> Self {
        Self::new(0)
    }
}