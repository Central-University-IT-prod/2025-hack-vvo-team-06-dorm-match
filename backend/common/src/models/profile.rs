use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::Type, ToSchema, Clone)]
#[sqlx(type_name = "user_sex", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Sex {
  Male,
  Female,
}

#[derive(Serialize, Deserialize, FromRow, Clone, PartialEq, ToSchema)]
pub struct StudentProfile {
  pub user_id: uuid::Uuid,
  pub faculty: String,
  pub course: i32,
  pub gender: String,
  pub age: i32,
  pub wake_hours: String,
  pub hobbies: serde_json::Value,
  pub mbti: Option<String>,
  pub updated_at: chrono::DateTime<chrono::Utc>,
}
