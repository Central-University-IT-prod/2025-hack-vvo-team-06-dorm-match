use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use dormmatch_common::{config::env::Config, utils::jwt::verify_jwt};

pub async fn jwt_middleware(
  req: ServiceRequest,
  credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
  let config = req
    .app_data::<actix_web::web::Data<Config>>()
    .expect("Config not found in app data");

  match verify_jwt(credentials.token(), &config.jwt_secret) {
    Ok(claims) => {
      req.extensions_mut().insert(claims);
      Ok(req)
    }
    Err(_) => Err(actix_web::error::ErrorUnauthorized("Invalid token")),
  }
}
