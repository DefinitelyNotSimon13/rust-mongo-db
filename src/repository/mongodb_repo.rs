use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::user_model::User;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{InsertOneResult, UpdateResult, DeleteResult},
    sync::{Client, Collection},
};

pub struct MongoRepo {
    user_collection: Collection<User>,
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
        MongoRepo { user_collection }
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_document = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let new_user = self
            .user_collection
            .insert_one(new_document, None)
            .expect("Error creating user");
        Ok(new_user)
    }

    pub fn get_user(&self, id: &String) -> Result<User, Error> {
        let object_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": object_id};
        let user_details = self
            .user_collection
            .find_one(filter, None)
            .expect("Error getting user's details!");
        Ok(user_details.unwrap())
    }

    pub fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let object_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": object_id};
        let new_doc = doc! {
            "$set":
            {
                "id": new_user.id,
                "name": new_user.name,
                "location": new_user.location,
                "title": new_user.title
            },
        };
        let updated_doc = self
            .user_collection
            .update_one(filter, new_doc, None)
            .expect("Error updating user!");
        Ok(updated_doc)
    }

    pub fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let object_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": object_id};
        let user_details = self
            .user_collection
            .delete_one(filter, None)
            .expect("Error deleting user");
        Ok(user_details)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let cursors = self
            .user_collection
            .find(None, None)
            .expect("Error getting list of users");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }


}
