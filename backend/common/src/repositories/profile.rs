use async_trait::async_trait;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::profile::{Sex, StudentProfile};
use crate::types::types::{MbtiType, WakeType};

#[async_trait]
pub trait StudentProfileRepository {
  async fn create(
    &self,
    pool: &PgPool,
    user_id: &Uuid,
    faculty: &str,
    course: i32,
    gender: Sex,
    age: i32,
    wake_hours: WakeType,
    hobbies: Vec<String>,
    mbti: Option<MbtiType>,
  ) -> Result<StudentProfile, sqlx::Error>;

  async fn get_by_user_id(
    &self,
    pool: &PgPool,
    user_id: &Uuid,
  ) -> Result<Option<StudentProfile>, sqlx::Error>;

  async fn find_by_user_id(
    pool: &PgPool,
    user_id: &uuid::Uuid,
  ) -> Result<Option<StudentProfile>, sqlx::Error> {
    sqlx::query_as!(
      StudentProfile,
      r#"
            SELECT 
                user_id, 
                faculty, 
                course, 
                gender AS "gender: _", 
                age, 
                wake_hours AS "wake_hours: _", 
                hobbies, 
                mbti AS "mbti: _", 
                updated_at
            FROM student_profiles 
            WHERE user_id = $1
            "#,
      user_id
    )
    .fetch_optional(pool)
    .await
  }

  async fn update(
    &self,
    pool: &PgPool,
    user_id: &Uuid,
    faculty: Option<String>,
    course: Option<i32>,
    gender: Option<Sex>,
    age: Option<i32>,
    wake_hours: Option<WakeType>,
    hobbies: Option<Vec<String>>,
    mbti: Option<MbtiType>,
  ) -> Result<StudentProfile, sqlx::Error>;
}

pub struct PostgresStudentProfileRepository;

#[async_trait]
impl StudentProfileRepository for PostgresStudentProfileRepository {
  async fn create(
    &self,
    pool: &PgPool,
    user_id: &Uuid,
    faculty: &str,
    course: i32,
    gender: Sex,
    age: i32,
    wake_hours: WakeType,
    hobbies: Vec<String>,
    mbti: Option<MbtiType>,
  ) -> Result<StudentProfile, sqlx::Error> {
    sqlx::query_as::<_, StudentProfile>(
            r#"
            INSERT INTO student_profiles (user_id, faculty, course, gender, age, wake_hours, hobbies, mbti, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(faculty)
        .bind(course)
        .bind(gender)
        .bind(age)
        .bind(wake_hours)
        .bind(json!(hobbies))
        .bind(mbti)
        .fetch_one(pool)
        .await
  }

  async fn get_by_user_id(
    &self,
    pool: &PgPool,
    user_id: &Uuid,
  ) -> Result<Option<StudentProfile>, sqlx::Error> {
    sqlx::query_as::<_, StudentProfile>(r#"SELECT * FROM student_profiles WHERE user_id = $1"#)
      .bind(user_id)
      .fetch_optional(pool)
      .await
  }

  async fn update(
    &self,
    pool: &PgPool,
    user_id: &Uuid,
    faculty: Option<String>,
    course: Option<i32>,
    gender: Option<Sex>,
    age: Option<i32>,
    wake_hours: Option<WakeType>,
    hobbies: Option<Vec<String>>,
    mbti: Option<MbtiType>,
  ) -> Result<StudentProfile, sqlx::Error> {
    sqlx::query_as::<_, StudentProfile>(
      r#"
            UPDATE student_profiles
            SET
                faculty = COALESCE($2, faculty),
                course = COALESCE($3, course),
                gender = COALESCE($4, gender),
                age = COALESCE($5, age),
                wake_hours = COALESCE($6, wake_hours),
                hobbies = COALESCE($7, hobbies),
                mbti = COALESCE($8, mbti),
                updated_at = NOW()
            WHERE user_id = $1
            RETURNING *
            "#,
    )
    .bind(user_id)
    .bind(faculty)
    .bind(course)
    .bind(gender)
    .bind(age)
    .bind(wake_hours)
    .bind(hobbies.map(|h| json!(h)))
    .bind(mbti)
    .fetch_one(pool)
    .await
  }
}
