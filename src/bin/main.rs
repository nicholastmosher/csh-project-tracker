use diesel::prelude::*;

use csh_project_tracker::{
    models::*,
    establish_connection,
};

fn main() {
    use csh_project_tracker::schema::projects::dsl::*;

    let connection = establish_connection();
    let results = projects.load::<Project>(&connection)
        .expect("Error loading projects");

    for project in results {
        println!("{:?}", project);
    }

    let sys = actix::System::new("csh-project-tracker");
    let _ = csh_project_tracker::app::launch();
    let _ = sys.run();
}
