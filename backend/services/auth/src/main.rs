use actix_web::{web, App, HttpServer};
use dormmatch_common::config::env::Config;
use sqlx::PgPool;
use tracing_subscriber;

mod config;
mod controllers;
mod middleware;
mod openapi;
mod services;

fn configure_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/auth")
      .route(
        "/register",
        web::post().to(controllers::auth::register_student),
      )
      .route("/login", web::post().to(controllers::auth::login))
      .route(
        "/verify",
        web::post().to(controllers::verify::verify_student),
      ),
  );

  openapi::configure_openapi(cfg);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  tracing_subscriber::fmt::init();
  let config = Config::from_env();
  let port_auth = config.port_auth; // Store port_auth before moving config

  let pool = PgPool::connect(&config.database_url)
    .await
    .expect("Failed to connect to PostgreSQL");

  println!("Server started!");

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(pool.clone()))
      .app_data(web::Data::new(config.clone()))
      .configure(configure_routes)
      .configure(openapi::configure_openapi)
  })
  .bind(("0.0.0.0", port_auth))?
  .run()
  .await
}
