use actix::{Message, Handler};
use crate::app::projects::CreateProject;
use crate::db::DbExecutor;
use crate::models::Project;
use diesel::RunQueryDsl;

impl Message for CreateProject {
    type Result = Result<(), String>;
}

impl Handler<CreateProject> for DbExecutor {
    type Result = <CreateProject as Message>::Result;

    fn handle(&mut self, msg: CreateProject, _: &mut Self::Context) -> Self::Result {
        use crate::schema::projects;
        let conn = &self.0.get().expect("should get db connection");

        let new_project = Project {
            title: msg.project_name,
            description: None,
            project_id: 0,
        };

        diesel::insert_into(projects::table)
            .values(&new_project)
            .get_result::<Project>(conn)
            .expect("should insert project");

        Ok(())
    }
}