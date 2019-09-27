use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use crate::schema::projects;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
pub struct Project {
    pub project_id: i32,
    pub title: String,
    pub description: Option<String>,
}
