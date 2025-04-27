//! src/startup.rs

use crate::{
    email_client::EmailClient,
    routes::{health_check, subscribe},
};
use actix_web::{App, HttpServer, dev::Server, web};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let db_pool = web::Data::new(db_pool);
    let email_client = web::Data::new(email_client);
    // Http layer handles the transport layer.
    // have to move the connection into the closure.
    let server = HttpServer::new(move || {
        // App is the application logic, routing, middleware, request handlers, etc.
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
