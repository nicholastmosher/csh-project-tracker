use actix_web::{App, web, Responder, HttpServer};
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

    let _ = http_server();
}

fn index() -> impl Responder {
    "Hello, what the shit!"
}

fn http_server() -> std::io::Result<()> {
    let mut listenfd = listenfd::ListenFd::from_env();

    let mut server = HttpServer::new(||
        App::new().service(
            web::scope("/api/v1")
                .route("/index.html", web::get().to(index)))
    );

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().unwrap();
    Ok(())
}
