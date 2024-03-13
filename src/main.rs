mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};

use api::user_api::{create_user, delete_user, get_all_users, get_user, update_user};
use repository::mongodb_repo::MongoRepo;

#[get("/")]
fn index() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello, world! Rust ist kinda cool!")))
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build().manage(db).mount(
        "/",
        routes![
            index,
            create_user,
            get_user,
            update_user,
            delete_user,
            get_all_users
        ],
    )
}
