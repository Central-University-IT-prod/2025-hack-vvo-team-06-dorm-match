use actix_web::{web, HttpResponse, Responder};
use dormmatch_common::{
    models::{application::Application, room::Room},
    repositories::{
        application::ApplicationRepository,
        profile::{PostgresStudentProfileRepository, StudentProfileRepository},
        room::RoomRepository,
        user::UserRepository,
    },
};
use serde_json::Value;
use sqlx::{types::chrono::Utc, PgPool};
use uuid::Uuid;
use crate::services::matching::MatchingService;
use crate::models::RoomStats;

#[utoipa::path(
    post,
    path = "/rooms",
    request_body(content = Room, content_type = "application/json"),
    responses(
        (status = 201, description = "Room created", body = Room),
        (status = 400, description = "Invalid input", body = String)
    )
)]
pub async fn create_room(room: web::Json<Room>, pool: web::Data<PgPool>) -> impl Responder {
    let room = room.into_inner();
    match RoomRepository::create(&pool, &room).await {
        Ok(room) => HttpResponse::Created().json(room),
        Err(e) => HttpResponse::BadRequest().body(format!("Failed to create room: {}", e)),
    }
}

#[utoipa::path(
    get,
    path = "/rooms/search",
    responses(
        (status = 200, description = "List of available rooms", body = [Room]),
        (status = 400, description = "Invalid parameters", body = String)
    )
)]
pub async fn search_rooms(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<uuid::Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = user_id.into_inner();
    let user = UserRepository::find_by_id(&pool, &user_id)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    if user.is_none() {
        return Err(actix_web::error::ErrorNotFound("User not found"));
    }

    let profile = PostgresStudentProfileRepository
        .get_by_user_id(&pool, &user_id)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let profile = profile.ok_or_else(|| actix_web::error::ErrorBadRequest("Profile not found"))?;

    let rooms = RoomRepository::find_available(
        &pool,
        Some(&profile.faculty),
        Some(profile.course),
        &profile.gender,
    )
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(rooms))
}

#[utoipa::path(
    post,
    path = "/rooms/apply",
    request_body(content = Value, content_type = "application/json"),
    responses(
        (status = 201, description = "Application submitted", body = Application),
        (status = 400, description = "Invalid input", body = String)
    )
)]
pub async fn apply_room(
    body: web::Json<Value>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = body
        .get("user_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Invalid user_id"))?;

    let room_id = body
        .get("room_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Invalid room_id"))?;

    let application = Application {
        id: Uuid::new_v4(),
        user_id,
        room_id,
        status: "pending".to_string(),
        comment: None,
        created_at: Utc::now(),
    };

    let app = ApplicationRepository::create(&pool, &application)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to apply: {}", e)))?;

    Ok(HttpResponse::Created().json(app))
}

#[utoipa::path(
    get,
    path = "/rooms/applications",
    params(
        ("user_id", Query, description = "User ID")
    ),
    responses(
        (status = 200, description = "List of applications", body = [Application]),
        (status = 400, description = "Invalid user_id", body = String)
    )
)]
pub async fn get_applications(
    query: web::Query<Value>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = query
        .get("user_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Invalid user_id"))?;

    let apps = ApplicationRepository::find_by_user_id(&pool, &user_id)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to get applications: {}", e)))?;

    Ok(HttpResponse::Ok().json(apps))
}

#[utoipa::path(
    post,
    path = "/rooms/applications/{id}/approve",
    params(
        ("id", Path, description = "Application ID")
    ),
    request_body(content = Value, content_type = "application/json"),
    responses(
        (status = 200, description = "Application approved", body = Application),
        (status = 400, description = "Invalid input", body = String)
    )
)]
pub async fn approve_application(
    path: web::Path<Uuid>,
    body: web::Json<Value>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let id = path.into_inner();
    let comment = body
        .get("comment")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    match ApplicationRepository::update_status(&pool, &id, "approved", comment).await {
        Ok(app) => HttpResponse::Ok().json(app),
        Err(e) => HttpResponse::BadRequest().body(format!("Failed to approve: {}", e)),
    }
}

#[utoipa::path(
    post,
    path = "/rooms/applications/{id}/reject",
    params(
        ("id", Path, description = "Application ID")
    ),
    request_body(content = Value, content_type = "application/json"),
    responses(
        (status = 200, description = "Application rejected", body = Application),
        (status = 400, description = "Invalid input", body = String)
    )
)]
pub async fn reject_application(
    path: web::Path<Uuid>,
    body: web::Json<Value>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let id = path.into_inner();
    let comment = body
        .get("comment")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    match ApplicationRepository::update_status(&pool, &id, "rejected", comment).await {
        Ok(app) => HttpResponse::Ok().json(app),
        Err(e) => HttpResponse::BadRequest().body(format!("Failed to reject: {}", e)),
    }
}

#[utoipa::path(
    get,
    path = "/rooms/stats",
    responses(
        (status = 200, description = "Statistics", body = RoomStats)
    )
)]
pub async fn get_stats(pool: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let stats = sqlx::query!(
        r#"
        SELECT
            (SELECT COUNT(*) FROM rooms WHERE status = 'available') as available,
            (SELECT COUNT(*) FROM rooms WHERE status = 'occupied') as occupied,
            (SELECT COUNT(*) FROM rooms WHERE status = 'reserved') as reserved,
            (SELECT COUNT(*) FROM applications WHERE status = 'pending') as pending_applications
        "#
    )
    .fetch_one(&**pool)
    .await
    .map(|r| RoomStats {
        available_rooms: r.available.unwrap_or(0),
        occupied_rooms: r.occupied.unwrap_or(0),
        reserved_rooms: r.reserved.unwrap_or(0),
        pending_applications: r.pending_applications.unwrap_or(0),
    })
    .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to get stats: {}", e)))?;

    Ok(HttpResponse::Ok().json(stats))
}

#[utoipa::path(
    post,
    path = "/rooms/auto-assign",
    request_body(content = Value, content_type = "application/json"),
    responses(
        (status = 200, description = "Auto-assigned room", body = Application),
        (status = 400, description = "No suitable room found", body = String)
    )
)]
pub async fn auto_assign(
    body: web::Json<Value>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user_id = body
        .get("user_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Invalid user_id"))?;

    let user = UserRepository::find_by_id(&pool, &user_id)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("User lookup failed: {}", e)))?
        .ok_or_else(|| actix_web::error::ErrorNotFound("User not found"))?;

    let profile = PostgresStudentProfileRepository
        .get_by_user_id(&pool, &user.id)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Profile lookup failed: {}", e)))?
        .ok_or_else(|| actix_web::error::ErrorBadRequest("Profile not found"))?;

    let rooms = RoomRepository::find_available(
        &pool,
        Some(&profile.faculty),
        Some(profile.course),
        &profile.gender,
    )
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Room search failed: {}", e)))?;

    let matching_room = MatchingService::find_best_room(&profile, &rooms)
        .ok_or_else(|| actix_web::error::ErrorBadRequest("No suitable room found"))?;

    let application = Application {
        id: Uuid::new_v4(),
        user_id,
        room_id: matching_room.id,
        status: "pending".to_string(),
        comment: Some("Auto-assigned".to_string()),
        created_at: Utc::now(),
    };

    let app = ApplicationRepository::create(&pool, &application)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Failed to auto-assign: {}", e)))?;

    Ok(HttpResponse::Ok().json(app))
}
