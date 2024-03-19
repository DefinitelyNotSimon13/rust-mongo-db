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

#[get("/project/all")]
pub fn get_all_projects(db: &State<MongoRepo>) -> Result<Json<Vec<Project>>, Status> {
    let projects = db.get_all_projects();
    match projects {
        Ok(projects) => Ok(Json(projects)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/project/<path>")]
pub fn get_project(db: &State<MongoRepo>, path: String) -> Result<Json<Project>, Status> {
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

#[put("/project/<path>", data = "<new_project>")]
pub fn update_project(
    db: &State<MongoRepo>,
    path: String,
    new_project: Json<Project>,
) -> Result<Json<Project>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let data = Project {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        index: new_project.index.to_owned(),
        name: new_project.name.to_owned(),
        description: new_project.description.to_owned(),
        language: new_project.language.to_owned(),
        major_version: new_project.major_version.to_owned(),
        minor_version: new_project.minor_version.to_owned(),
        patch_version: new_project.patch_version.to_owned(),
        category: new_project.category.to_owned(),
        github_repo: new_project.github_repo.to_owned(),
    };
    let update_result = db.update_project(&id, data);
    let mut update_count: Option<u64> = None;
    match update_result {
        Ok(update) => update_count = Some(update.matched_count),
        Err(_) => update_count = None,
    };

    if update_count.is_none() {
        return Err(Status::InternalServerError);
    }

    if update_count == Some(0) {
        return Err(Status::NotFound);
    }

    let updated_project_info = db.get_project(&id);
    match updated_project_info {
        Ok(project) => Ok(Json(project)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/projec/<path>")]
pub fn delete_project(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    }
    let result = db.delete_project(&id);
    let mut deleted_count: Option<u64> = None;
    match result {
        Ok(res) => deleted_count = Some(res.deleted_count),
        Err(_) => deleted_count = None,
    }
    if deleted_count.is_none() {
        return Err(Status::InternalServerError);
    }

    if deleted_count == Some(0) {
        return Err(Status::NotFound);
    }

    Ok(Json("User successfully deleted!"))
}
