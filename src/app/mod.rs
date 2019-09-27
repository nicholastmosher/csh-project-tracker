use actix::{Addr, SyncArbiter};
use actix_web::{App, web, HttpServer, Responder};
use crate::db::DbExecutor;

pub mod projects;

pub struct AppState {
    db: Addr<DbExecutor>,
}

pub fn launch<S: Into<String>>(database_url: S) -> std::io::Result<()> {
    let database_url = database_url.into();
    let mut listenfd = listenfd::ListenFd::from_env();

    let database_addr = SyncArbiter::start(
        num_cpus::get(),
        move || DbExecutor::new(database_url.clone()),
    );

    let mut server = HttpServer::new(move ||
        App::new()
            .data(AppState { db: database_addr.clone() })
            .service(web::scope("/api/v1")
                .route("/projects", web::post().to_async(projects::create))
            )
    );

    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run()?;
    Ok(())
}
