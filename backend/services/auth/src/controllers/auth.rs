use actix_web::{web, HttpResponse, Responder};
use dormmatch_common::{
  models::{
    profile::Sex,
    user::{User, UserRole, UserStatus},
  },
  repositories::{
    profile::{PostgresStudentProfileRepository, StudentProfileRepository},
    user::UserRepository,
  },
  types::types::{MbtiType, WakeType},
  utils::{
    crypto::{hash_password, verify_password},
    jwt::create_jwt,
  },
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct RegisterStudentRequest {
  email: String,
  password: String,
  faculty: String,
  course: i32,
  gender: Sex,
  age: i32,
  wake_hours: WakeType,
  hobbies: Vec<String>,
  mbti: Option<MbtiType>,
}

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterStudentRequest,
    responses(
        (status = 201, description = "User registered successfully", body = User),
        (status = 400, description = "User already exists", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
pub async fn register_student(
  req: web::Json<RegisterStudentRequest>,
  pool: web::Data<PgPool>,
  _config: web::Data<dormmatch_common::config::env::Config>,
) -> impl Responder {
  let password_hash = match hash_password(&req.password) {
    Ok(hash) => hash,
    Err(_) => return HttpResponse::InternalServerError().body("Failed to hash password"),
  };

  let user = UserRepository::create(
    &pool,
    &req.email,
    &password_hash,
    UserRole::Student,
    UserStatus::Pending,
  )
  .await;

  match user {
    Ok(user) => {
      let profile = PostgresStudentProfileRepository
        .create(
          &pool,
          &user.id,
          &req.faculty,
          req.course,
          req.gender.clone(),
          req.age,
          req.wake_hours.clone(),
          req.hobbies.clone(),
          req.mbti.clone(),
        )
        .await;

      match profile {
        Ok(_) => HttpResponse::Created().json(user),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create profile"),
      }
    }
    Err(_) => HttpResponse::BadRequest().body("User already exists"),
  }
}

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
  email: String,
  password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
  token: String,
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials or user not found", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
pub async fn login(
  req: web::Json<LoginRequest>,
  pool: web::Data<PgPool>,
  config: web::Data<dormmatch_common::config::env::Config>,
) -> impl Responder {
  let user = UserRepository::find_by_email(&pool, &req.email).await;

  match user {
    Ok(Some(user)) => {
      if verify_password(&req.password, &user.password_hash).is_ok() {
        let role = match user.role.as_str() {
          "student" => "student",
          "admin" => "admin",
          _ => return HttpResponse::InternalServerError().body("Invalid role"),
        };
        match create_jwt(&user.id.to_string(), role, &config.jwt_secret) {
          Ok(token) => HttpResponse::Ok().json(LoginResponse { token }),
          Err(_) => HttpResponse::InternalServerError().body("Failed to create JWT"),
        }
      } else {
        HttpResponse::Unauthorized().body("Invalid credentials")
      }
    }
    Ok(None) => HttpResponse::Unauthorized().body("User not found"),
    Err(_) => HttpResponse::InternalServerError().body("Database error"),
  }
}
