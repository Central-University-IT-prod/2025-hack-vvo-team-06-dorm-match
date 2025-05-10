use crate::models::user::{User, UserRole, UserStatus};
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
  pub async fn create(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
    role: UserRole,
    status: UserStatus,
  ) -> Result<User, Error> {
    sqlx::query_as::<_, User>(
      r#"
            INSERT INTO users (id, email, password_hash, role, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
            RETURNING *
            "#,
    )
    .bind(Uuid::new_v4())
    .bind(email)
    .bind(password_hash)
    .bind(role)
    .bind(status)
    .fetch_one(pool)
    .await
  }

  pub async fn find_by_id(pool: &PgPool, id: &uuid::Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, role AS "role: _", status AS "status: _", created_at, updated_at
            FROM users WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
  }

  pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, Error> {
    sqlx::query_as::<_, User>(r#"SELECT * FROM users WHERE email = $1"#)
      .bind(email)
      .fetch_optional(pool)
      .await
  }

  pub async fn update_status(
    pool: &PgPool,
    user_id: Uuid,
    status: UserStatus,
  ) -> Result<User, Error> {
    sqlx::query_as::<_, User>(
      r#"
            UPDATE users
            SET status = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING *
            "#,
    )
    .bind(status)
    .bind(user_id)
    .fetch_one(pool)
    .await
  }
}
