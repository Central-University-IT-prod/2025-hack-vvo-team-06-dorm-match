use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, FromRow, ToSchema, Clone)]
pub struct Application {
  pub id: uuid::Uuid,
  pub user_id: uuid::Uuid,
  pub room_id: uuid::Uuid,
  pub status: String,
  pub comment: Option<String>,
  pub created_at: DateTime<Utc>,
}
