use crate::models::user_model::User;
use crate::repository::mongo_repo::MongoRepo;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
};

impl MongoRepo {
    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let document_count: i64 = match self.user_collection.estimated_document_count(None) {
            Ok(count) => count.try_into().unwrap(),
            Err(_) => -1,
        };
        if document_count == -1 {
            panic!("Error counting documents");
        }

        let new_document = User {
            id: None,
            index: Some(document_count + 1),
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
                "index": new_user.index,
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
