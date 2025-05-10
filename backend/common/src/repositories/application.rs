use sqlx::PgPool;
use crate::models::application::Application;

pub struct ApplicationRepository;

impl ApplicationRepository {
    pub async fn create(pool: &PgPool, app: &Application) -> Result<Application, sqlx::Error> {
        sqlx::query_as!(
            Application,
            r#"
            INSERT INTO applications (id, user_id, room_id, status, comment, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, user_id, room_id, status, comment, created_at
            "#,
            app.id,
            app.user_id,
            app.room_id,
            &app.status,
            app.comment,
            app.created_at
        )
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_user_id(
        pool: &PgPool,
        user_id: &uuid::Uuid,
    ) -> Result<Vec<Application>, sqlx::Error> {
        sqlx::query_as!(
            Application,
            r#"
            SELECT id, user_id, room_id, status, comment, created_at
            FROM applications WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn update_status(
        pool: &PgPool,
        id: &uuid::Uuid,
        status: &str,
        comment: Option<String>,
    ) -> Result<Application, sqlx::Error> {
        sqlx::query_as!(
            Application,
            r#"
            UPDATE applications
            SET status = $1, comment = $2
            WHERE id = $3
            RETURNING id, user_id, room_id, status, comment, created_at
            "#,
            status,
            comment,
            id
        )
        .fetch_one(pool)
        .await
    }
}
