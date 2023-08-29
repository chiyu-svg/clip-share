
pub mod domain;
pub mod data;
pub mod service;
pub mod web;
use data::AppDataBase;
use rocket::http::Header;
use rocket::{Rocket, Build, Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub fn get_rocket(config: RocketConfig) -> Rocket<Build> {
    rocket::build()
        .attach(CORS)
        .manage::<AppDataBase>(config.database)
        .mount("/", web::httpd::routes())
        .register("/", web::httpd::catcher::catchers())
}

struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "GET/POST Counter",
            kind: Kind::Response
        }
    }
    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        res.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}



pub struct RocketConfig {
    pub database: AppDataBase
}