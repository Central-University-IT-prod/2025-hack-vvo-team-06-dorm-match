use utoipa::{
  openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
  Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

use crate::controllers::{
  auth::{LoginRequest, LoginResponse, RegisterStudentRequest},
  verify::VerifyStudentRequest,
};
use dormmatch_common::models::{profile::StudentProfile, user::User};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::controllers::auth::register_student,
        crate::controllers::auth::login,
        crate::controllers::verify::verify_student,
    ),
    components(
        schemas(
            User,
            StudentProfile,
            RegisterStudentRequest,
            LoginRequest,
            LoginResponse,
            VerifyStudentRequest,
        )
    ),
    modifiers(&SecurityAddon),
    info(
        title = "DormMatch Auth API",
        description = "API for user authentication and profile management in DormMatch",
        version = "0.1.0",
        contact(
            name = "DormMatch Team",
            email = "support@dormmatch.com"
        )
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
  fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
    let components = openapi
      .components
      .get_or_insert_with(utoipa::openapi::Components::new);
    components.add_security_scheme(
      "bearerAuth",
      SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
    );
  }
}

pub fn configure_openapi(cfg: &mut actix_web::web::ServiceConfig) {
  cfg
    .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()));
}
