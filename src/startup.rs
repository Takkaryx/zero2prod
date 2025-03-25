//! src/startup.rs

use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // Http layer handles the transport layer.
    let server = HttpServer::new(|| {
        // App is the application logic, routing, middleware, request handlers, etc.
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
