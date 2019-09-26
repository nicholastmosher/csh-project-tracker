use std::env;
use actix::{Addr, SyncArbiter};
use actix_web::{App, web, HttpServer, Responder};
use crate::db::{self, DbExecutor};

pub struct AppState {
    db: Addr<DbExecutor>,
}

fn index() -> impl Responder {
    "Hello, what the shit!"
}

pub fn launch() -> std::io::Result<()> {
    let mut listenfd = listenfd::ListenFd::from_env();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_addr = SyncArbiter::start(
        num_cpus::get(),
        move || DbExecutor::new(database_url.clone()),
    );

    let mut server = HttpServer::new(move ||
        App::new()
            .data(AppState { db: database_addr.clone() })
            .service(web::scope("/api/v1")
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
