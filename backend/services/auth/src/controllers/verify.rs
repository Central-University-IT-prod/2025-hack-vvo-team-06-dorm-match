use actix_web::{web, HttpResponse, Responder};
use dormmatch_common::{models::user::UserStatus, repositories::user::UserRepository};
use serde::Deserialize;
use sqlx::PgPool;
use utoipa::ToSchema;
use uuid::Uuid;

use dormmatch_common::models::user::User;

#[derive(Deserialize, ToSchema)]
pub struct VerifyStudentRequest {
  user_id: Uuid,
  is_verified: bool,
}

#[utoipa::path(
    post,
    path = "/auth/verify",
    request_body = VerifyStudentRequest,
    responses(
        (status = 200, description = "User status updated", body = User),
        (status = 404, description = "User not found", body = String)
    )
)]
pub async fn verify_student(
  req: web::Json<VerifyStudentRequest>,
  pool: web::Data<PgPool>,
) -> impl Responder {
  let status = if req.is_verified {
    UserStatus::Verified
  } else {
    UserStatus::Rejected
  };

  let result = UserRepository::update_status(&pool, req.user_id, status).await;

  match result {
    Ok(user) => HttpResponse::Ok().json(user),
    Err(_) => HttpResponse::NotFound().body("User not found"),
  }
}
