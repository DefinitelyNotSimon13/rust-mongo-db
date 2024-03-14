use crate::models::project_model::Project;
use crate::repository::mongo_repo::MongoRepo;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
};

impl MongoRepo {
    pub fn create_project(&self, new_project: Project) -> Result<InsertOneResult, Error> {
        let document_count: i64 = match self.project_collection.estimated_document_count(None) {
            Ok(count) => count.try_into().unwrap(),
            Err(_) => -1,
        };
        if document_count == -1 {
            panic!("Error counting documents");
        }

        let new_document = Project {
            id: None,
            index: Some(document_count + 1),
            name: new_project.name,
            description: new_project.description,
            language: new_project.language,
            major_version: new_project.major_version,
            minor_version: new_project.minor_version,
            patch_version: new_project.patch_version,
            category: new_project.category,
            github_repo: new_project.github_repo,
        };

        let new_project = self
            .project_collection
            .insert_one(new_document, None)
            .expect("Error creating project");
        Ok(new_project)
    }

    pub fn get_project(&self, id: &String) -> Result<Project, Error> {
        let object_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": object_id};
        let project_details = self
            .project_collection
            .find_one(filter, None)
            .expect("Error getting project details!");
        Ok(project_details.unwrap())
    }

    pub fn update_project(&self, id: &String, new_project: Project) -> Result<UpdateResult, Error> {
        let object_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": object_id};
        let new_doc = doc! {
            "$set":
            {
                "id": new_project.id,
                "index": new_project.index,
                "name": new_project.name,
                "description": new_project.description,
                "language": new_project.language,
                "major_version": new_project.major_version,
                "minor_version": new_project.minor_version,
                "patch_version": new_project.patch_version,
                "category": new_project.category,
                "github_repo": new_project.github_repo
            },
        };

        let updated_doc = self
            .project_collection
            .update_one(filter, new_doc, None)
            .expect("Error updating project!");
        Ok(updated_doc)
    }

    pub fn delete_project(&self, id: &String) -> Result<DeleteResult, Error> {
        let object_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": object_id};
        let project_details = self
            .project_collection
            .delete_one(filter, None)
            .expect("Error deleting project!");
        Ok(project_details)
    }

    pub fn get_all_projects(&self) -> Result<Vec<Project>, Error> {
        let cursors = self
            .project_collection
            .find(None, None)
            .expect("Error getting list of projects!");
        let projects = cursors.map(|doc| doc.unwrap()).collect();
        Ok(projects)
    }
}
