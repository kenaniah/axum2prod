use axum::Form;
use axum::{http::StatusCode, response::IntoResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(Form(_data): Form<FormData>) -> impl IntoResponse {
    StatusCode::CREATED
}
