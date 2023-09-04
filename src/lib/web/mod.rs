pub mod httpd;
pub mod form;

pub const PASSWORD_COOKIE: &str = "password";

#[derive(rocket::Responder, Debug)]
pub enum PageError {
    #[response(status = 500)]
    Serialzation(String),
    #[response(status = 500)]
    Render(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 401)]
    NoPermissions(String),
    #[response(status = 500)]
    Internal(String),
    #[response(status = 500)]
    ParseError(String)
}

impl From<serde_json::Error> for PageError {
    fn from(err: serde_json::Error) -> Self {
        PageError::Serialzation(format!("{}", err))
    }
}