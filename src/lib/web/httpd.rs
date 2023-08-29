use rocket::response::{ status, Redirect };
use rocket::serde::json::Json;
use rocket::{ uri, State, Request };
use rocket::http::Status;
use rocket::form::{ Form, Contextual };
use crate::service::ask;
use super::PageError;
use crate::domain::{ShortCode, clip};
use crate::data::AppDataBase;
use crate::service::action;
use crate::domain::Clip;
use super::form;


#[rocket::get("/clip/<shortcode>")]
pub async fn get_clip(
    shortcode: ShortCode,
    database: &State<AppDataBase>
) -> Result<status::Custom<Json<Clip>>, PageError> {
    match action::get_clip(shortcode.into(), &database.get_pool()).await {
        Ok(clip) => Ok(status::Custom(Status::Ok, Json(clip))),
        Err(err) => Err(PageError::Internal("hanpped interal error".to_string()))
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
) -> Result<Redirect, PageError> {
    let clip = clip.into_inner();
    let new_clip: Result<ask::NewClip, PageError> = clip.try_into();
    match new_clip {
        Ok(new_clip) => {
            match action::new_clip(new_clip, database.get_pool()).await {
                        Ok(clip) =>  Ok(Redirect::to(uri!(get_clip(clip.shortcode)))),
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
    rocket::routes![get_clip, new_clip, update_clip, take_cors]
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