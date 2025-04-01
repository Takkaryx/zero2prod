//! src/startup.rs

use crate::routes::{health_check, subscribe};
use actix_web::{App, HttpServer, dev::Server, middleware::Logger, web};
use sqlx::PgPool;
use std::net::TcpListener;

pub async fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let db_pool = web::Data::new(db_pool);
    // Http layer handles the transport layer.
    // have to move the connection into the closure.
    let server = HttpServer::new(move || {
        // App is the application logic, routing, middleware, request handlers, etc.
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
