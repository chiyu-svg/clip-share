use derive_more::From;

/// the shortcode field for (`Clip`)[crate::domain::clip::Clip]
#[derive(Debug, From)]
pub struct ShortCode(String);

impl ShortCode {
    pub fn new() -> Self {
        use rand::prelude::*;
        let mut shortcode = String::with_capacity(10);
        let allowed_chars = ['b', 'c', 'd', 'e', 'f', 'g', '1', '2', '3', '5'];
        let mut rng = thread_rng();
        for _ in 0..10 {
            shortcode.push(
                *allowed_chars
                    .choose(&mut rng)
                    .expect("sampling file")
            );
        };
        Self(shortcode)
    }
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<ShortCode> for String {
    fn from(shortcode: ShortCode) -> Self {
        shortcode.0.clone()
    }
}
