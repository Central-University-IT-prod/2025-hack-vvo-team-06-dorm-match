use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, FromRow, ToSchema, Clone, PartialEq)]
pub struct Room {
  pub id: uuid::Uuid,
  pub number: String,
  pub description: String,
  pub photo_url: Option<String>,
  pub capacity: i32,
  pub current_occupants: i32,
  pub faculty_restriction: Option<String>,
  pub course_restriction: Option<i32>,
  pub sex_restriction: String,
  pub status: String,
}
