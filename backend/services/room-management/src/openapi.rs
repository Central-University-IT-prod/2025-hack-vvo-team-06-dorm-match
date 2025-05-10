use crate::models::RoomStats;
use dormmatch_common::models::room::Room;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
  paths(
    crate::controllers::rooms::create_room,
    crate::controllers::rooms::search_rooms,
    crate::controllers::rooms::apply_room,
    crate::controllers::rooms::get_applications,
    crate::controllers::rooms::approve_application,
    crate::controllers::rooms::reject_application,
    crate::controllers::rooms::get_stats,
    crate::controllers::rooms::auto_assign
  ),
  components(schemas(Room, RoomStats))
)]
pub struct ApiDoc;

pub fn configure_openapi(cfg: &mut actix_web::web::ServiceConfig) {
  cfg.service(
    utoipa_swagger_ui::SwaggerUi::new("/swagger-ui/{_:.*}")
      .url("/api-docs/openapi.json", ApiDoc::openapi()),
  );
}
