use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, FromRow, Clone, ToSchema)]
pub struct User {
  pub id: uuid::Uuid,
  pub email: String,
  pub password_hash: String,
  pub role: String,
  pub status: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "user_role", rename_all = "snake_case")]
pub enum UserRole {
  Student,
  Admin,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "user_status", rename_all = "snake_case")]
pub enum UserStatus {
  Pending,
  Verified,
  Rejected,
}
