use serde::Deserialize;
use actix_web::{web, HttpResponse};
use futures::Future;
use crate::app::AppState;

#[derive(Debug, Deserialize)]
pub struct In<T> {
    project: T,
}

#[derive(Debug, Deserialize)]
pub struct CreateProject {
    pub project_name: String,
}

pub fn create(
    state: web::Data<AppState>,
    post_project: web::Json<CreateProject>,
) -> impl Future<Item=HttpResponse, Error=()> {
    let db = &state.get_ref().db;
    let create_project = post_project.into_inner();

    db.send(create_project)
        .and_then(|res| match res {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(_) => Ok(HttpResponse::InternalServerError().finish()),
        })
        .map_err(|_| ())
}