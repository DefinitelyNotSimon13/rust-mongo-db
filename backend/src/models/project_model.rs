use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Project{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub index: Option<i64>,
    pub name: String,
    pub description: String,
    pub language: String,
    pub major_version: String,
    pub minor_version: String,
    pub patch_version: String,
    pub category: String,
    pub github_repo: String,
}
