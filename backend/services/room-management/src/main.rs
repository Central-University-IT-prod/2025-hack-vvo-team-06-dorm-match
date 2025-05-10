use actix_web::{web, App, HttpServer};
use dormmatch_common::config::env::Config;
use sqlx::PgPool;

mod controllers;
mod models;
mod openapi;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let config = Config::from_env();
  let pool = PgPool::connect(&config.database_url)
    .await
    .expect("Failed to connect to database");

  HttpServer::new(move || {
    App::new()
      .app_data(web::Data::new(pool.clone()))
      .service(
        web::scope("/rooms")
          .route("", web::post().to(controllers::rooms::create_room))
          .route("/search", web::get().to(controllers::rooms::search_rooms))
          .route("/apply", web::post().to(controllers::rooms::apply_room))
          .service(
            web::resource("/applications")
              .route(web::get().to(controllers::rooms::get_applications)),
          )
          .route("/stats", web::get().to(controllers::rooms::get_stats))
          .service(
            web::resource("/auto-assign").route(web::post().to(controllers::rooms::auto_assign)),
          )
          .service(
            web::resource("/applications/{id}/approve")
              .route(web::post().to(controllers::rooms::approve_application)),
          )
          .service(
            web::resource("/applications/{id}/reject")
              .route(web::post().to(controllers::rooms::reject_application)),
          ),
      )
      .configure(openapi::configure_openapi)
  })
  .bind(("0.0.0.0", config.port_room_management))?
  .run()
  .await
}
