use crate::models::room::Room;
use sqlx::PgPool;

pub struct RoomRepository;

impl RoomRepository {
    pub async fn create(pool: &PgPool, room: &Room) -> Result<Room, sqlx::Error> {
        sqlx::query_as!(
            Room,
            r#"
            INSERT INTO rooms (id, number, description, photo_url, capacity, current_occupants, faculty_restriction, course_restriction, sex_restriction, status)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, number, description, photo_url, capacity, current_occupants, faculty_restriction, course_restriction, sex_restriction, status
            "#,
            room.id,
            room.number,
            room.description,
            room.photo_url,
            room.capacity,
            room.current_occupants,
            room.faculty_restriction,
            room.course_restriction,
            &room.sex_restriction,
            &room.status,
        )
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: &uuid::Uuid) -> Result<Option<Room>, sqlx::Error> {
        sqlx::query_as!(
            Room,
            r#"
            SELECT id, number, description, photo_url, capacity, current_occupants, faculty_restriction, course_restriction, sex_restriction, status
            FROM rooms WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn find_available(
        pool: &PgPool,
        faculty: Option<&str>,
        course: Option<i32>,
        sex: &str,
    ) -> Result<Vec<Room>, sqlx::Error> {
        sqlx::query_as!(
            Room,
            r#"
            SELECT id, number, description, photo_url, capacity, current_occupants, faculty_restriction, course_restriction, sex_restriction, status
            FROM rooms
            WHERE status = 'available'
            AND (faculty_restriction IS NULL OR faculty_restriction = $1)
            AND (course_restriction IS NULL OR course_restriction = $2)
            AND (sex_restriction = $3 OR sex_restriction = 'any')
            "#,
            faculty,
            course,
            sex,
        )
        .fetch_all(pool)
        .await
    }
}
