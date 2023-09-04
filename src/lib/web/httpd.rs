use rocket::response::{ status, Redirect };
use rocket::serde::json::Json;
use rocket::{ uri, State, Request };
use rocket::http::{ Status, Cookie, CookieJar};
use rocket::form::{ Form, Contextual };
use crate::web::PASSWORD_COOKIE;
use super::PageError;
use crate::domain::{ShortCode, clip};
use crate::data::AppDataBase;
use crate::service::{ action, ask, ServiceError };
use crate::domain::Clip;
use crate::web::form;

#[rocket::get("/clip/<shortcode>")]
pub async fn get_clip(
    shortcode: ShortCode,
    database: &State<AppDataBase>
) -> Result<status::Custom<Json<Clip>>, PageError> {
    match action::get_clip(shortcode.into(), &database.get_pool()).await {
        Ok(clip) => Ok(status::Custom(Status::Ok, Json(clip))),
        Err(err) => match err {
            ServiceError::PermissionError(msg) => Err(PageError::NoPermissions(msg)),
            ServiceError::NotFound => Err(PageError::NotFound("Clip Not Found".to_string())),
            _ => Err(PageError::Internal("server error".to_owned()))
        }
    }
}

#[rocket::post("/clip/<shortcode>", format="json", data="<get_clip_password_protected>")]
pub async fn submit_clip_password(
    cookies: &CookieJar<'_>,
    shortcode: ShortCode,
    get_clip_password_protected: Json<form::GetClipPasswordProtected>,
    database: &State<AppDataBase>
) -> Result<status::Custom<Json<Clip>>, PageError> {
    let get_clip_password_protected = get_clip_password_protected.into_inner();
    let req = ask::GetClip {
        shortcode,
        password: get_clip_password_protected.password.clone()
    };
    match action::get_clip(req, database.get_pool()).await {
        Ok(clip) => {
            cookies.add(Cookie::new(
                PASSWORD_COOKIE,
                get_clip_password_protected.password.clone().into_inner().unwrap()
            ));
            Ok(status::Custom(Status::Ok, Json(clip)))
        }
        Err(e) => match e {
            ServiceError::PermissionError(e) => Err(PageError::NoPermissions(e.to_string())),
            ServiceError::NotFound => Err(PageError::NotFound("Clip not found".to_string())),
            _ => Err(PageError::Internal("server error".to_owned()))
        }
    }
}

#[rocket::options("/<_..>")]
pub async fn take_cors() -> &'static str {
    ""
}

#[rocket::post("/clip", format="json", data="<clip>")]
pub async fn new_clip(
    clip: Json<form::NewClipJson>,
    database: &State<AppDataBase>,
) -> Result<String, PageError> {
    let clip = clip.into_inner();
    let new_clip: Result<ask::NewClip, PageError> = clip.try_into();
    match new_clip {
        Ok(new_clip) => {
            match action::new_clip(new_clip, database.get_pool()).await {
                        Ok(clip) =>  Ok(clip.shortcode.into_inner()),
                        Err(_) => Err(PageError::Render("乱七把澡".to_string()))
            }
        },
        Err(e) => Err(e)
    }
}

#[rocket::post("/clip/update", format="json", data="<clip>")]
pub async fn update_clip(
    clip: Json<form::UpdateClipJson>,
    database: &State<AppDataBase>
) -> Result<Redirect, PageError> {
    let clip: Result<ask::UpdateClip, PageError> = clip.into_inner().try_into();
    match clip {
        Ok(update_clip) => {
            match action::update_clip(update_clip, database.get_pool()).await {
                Ok(clip) => Ok(Redirect::to(uri!(get_clip(clip.shortcode)))),
                Err(_) => Err(PageError::Internal("Database happend error".to_string()))
            }
        },
        Err(e) => Err(e) 
    }
    
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![get_clip, new_clip, update_clip, take_cors, submit_clip_password]
}
pub mod catcher {
    //! Contains all the page catchers.
    use rocket::Request;
    use rocket::{catch, catchers, Catcher};

    /// Catch unhandled errors.
    #[catch(default)]
    fn default(req: &Request) -> &'static str {
        eprintln!("General error: {:?}", req);
        "something went wrong..."
    }

    /// Catch server errors.
    #[catch(500)]
    fn internal_error(req: &Request) -> &'static str {
        eprintln!("Internal error: {:?}", req);
        "internal server error"
    }

    /// Catch missing data errors.
    #[catch(404)]
    fn not_found() -> &'static str {
        "404"
    }

    pub fn catchers() -> Vec<Catcher> {
        catchers![not_found, default, internal_error]
    }
}