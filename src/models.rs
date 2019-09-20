use serde::{Serialize, Deserialize};
use diesel::Queryable;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Project {
    project_id: i32,
    title: String,
    description: Option<String>,
}
