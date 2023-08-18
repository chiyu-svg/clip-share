use crate::domain::Time;


/// The file expires for (`Clip`)[crate::domain::clip::Clip]
#[derive(Debug)]
pub struct Expires(Option<Time>);

impl Expires {
    pub fn new<T: Into<Option<Time>>>(time: T) -> Self {
        let time: Option<Time> = time.into();
        match time {
            Some(time) => Expires(Some(time)),
            None => Expires(None)
        }
    }
}