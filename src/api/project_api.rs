use crate::{models::project_model::Project, repository::mongo_repo::MongoRepo};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};

#[post("/project/add", data = "<new_project>")]
pub fn create_project(
    db: &State<MongoRepo>,
    new_project: Json<Project>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Project {
        id: None,
        index: None,
        name: new_project.name.to_owned(),
        description: new_project.description.to_owned(),
        language: new_project.language.to_owned(),
        major_version: new_project.major_version.to_owned(),
        minor_version: new_project.minor_version.to_owned(),
        patch_version: new_project.patch_version.to_owned(),
        category: new_project.category.to_owned(),
        github_repo: new_project.github_repo.to_owned(),
    };
    let project_details = db.create_project(data);
    match project_details {
        Ok(project) => Ok(Json(project)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/project/<path>")]
pub fn get_project(db: &State<MongoRepo>, path:String) -> Result<Json<Project>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let project_details = db.get_project(&id);
    match project_details {
        Ok(project) => Ok(Json(project)),
        Err(_) => Err(Status::InternalServerError),
    }
}
