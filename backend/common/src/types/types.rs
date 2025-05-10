use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Type, Clone, ToSchema)]
#[sqlx(type_name = "wake_type", rename_all = "snake_case")]
pub enum WakeType {
  EarlyBird,
  NightOwl,
  Flexible,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone, ToSchema)]
#[sqlx(type_name = "mbti_type", rename_all = "snake_case")]
pub enum MbtiType {
  Intj,
  Intp,
  Entj,
  Entp,
  Infj,
  Infp,
  Enfj,
  Enfp,
  Istj,
  Isfj,
  Estj,
  Esfj,
  Istp,
  Isfp,
  Estp,
  Esfp,
}
