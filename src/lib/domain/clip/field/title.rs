// todo

/// The title for [`Clip`](crate::domain::clip::Clip)
#[derive(Debug)]
pub struct Title(Option<String>);

impl Title {
    pub fn new<T: Into<Option<String>>>(title: T) -> Self {
        let title: Option<String> = title.into();
        match title {
            Some(title) => {
                if !title.trim().is_empty() {
                    Title(Some(title))
                } else {
                    Title(None)
                }
            },
            None => Title(None)
        }
    }
}