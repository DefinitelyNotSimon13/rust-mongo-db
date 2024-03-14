use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::sync::{Client, Collection};

use crate::models::user_model::User;
use crate::models::project_model::Project;

pub struct MongoRepo {
    pub user_collection: Collection<User>,
    pub project_collection: Collection<Project>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => "Error loading env variable!".to_string(),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let user_collection: Collection<User> = db.collection("User");
        let project_collection: Collection<Project> = db.collection("Project");
        MongoRepo { user_collection, project_collection }
    }
}
