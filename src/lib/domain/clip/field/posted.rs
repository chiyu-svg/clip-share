use crate::domain::Time;
use derive_more::{Constructor, From};
use serde::{Serialize, Deserialize};


/// The posted field for (`Clip`)[crate::domain::clip::Clip]
#[derive(Debug, From, Constructor, Serialize, Deserialize)]
pub struct Posted(Time);

impl Posted {
    fn into_inner(self) -> Time {
        self.0
    }
}