use crate::Error;
use axum::http::StatusCode;
use axum::{extract::State, Form};
use chrono::Utc;
use uuid::Uuid;

use crate::AppContext;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(
    State(ctx): State<AppContext>,
    Form(form): Form<FormData>,
) -> Result<StatusCode, Error> {
    tracing::info!(
        "Adding '{}' '{}' as a new subscriber.",
        form.email,
        form.name
    );
    tracing::info!("Saving new subscriber details in the database...");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, name, email, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.name,
        form.email,
        Utc::now()
    )
    .execute(&ctx.db)
    .await
    {
        Ok(_) => {
            tracing::info!("New subscriber details have been saved");
            Ok(StatusCode::CREATED)
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            Err(e.into())
        }
    }
}
