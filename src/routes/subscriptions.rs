//! src/routes/subscriptions.rs

use actix_web::{HttpResponse, Responder, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name="Adding a new subscriber",
    skip(form, db_pool),
    fields(
        subscriber_email=%form.email,
        subscriber_name=%form.name)
)]
pub async fn subscribe(form: web::Form<FormData>, db_pool: web::Data<PgPool>) -> impl Responder {
    match insert_subscriber(&db_pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "saving new subscriber details into database", skip(form, pool))]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // We use 'get_ref' to get an immutable reference to the 'PgConnection'
    // wrapped by 'web::Data'
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
