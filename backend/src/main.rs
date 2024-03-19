mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;
use rocket::{
    fairing::{Fairing, Info, Kind},
    get,
    http::{Header, Status},
    serde::json::Json,
    Request, Response,
};

use api::{
    project_api::{create_project, delete_project, get_all_projects, get_project, update_project},
    user_api::{create_user, delete_user, get_all_users, get_user,get_user_by_name, update_user},
};
use repository::mongo_repo::MongoRepo;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

#[get("/")]
fn index() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello, world! Rust ist kinda cool!")))
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount(
            "/",
            routes![
                all_options,
                index,
                create_user,
                get_user,
                update_user,
                delete_user,
                get_all_users,
                get_user_by_name,
                create_project,
                get_project,
                get_all_projects,
                delete_project,
                update_project
            ],
        )
        .attach(CORS)
}
