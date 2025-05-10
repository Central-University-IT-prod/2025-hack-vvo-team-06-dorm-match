use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RoomStats {
  pub available_rooms: i64,
  pub occupied_rooms: i64,
  pub reserved_rooms: i64,
  pub pending_applications: i64,
}
