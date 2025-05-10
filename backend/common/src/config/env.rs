use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
  pub database_url: String,
  pub redis_url: String,
  pub jwt_secret: String,
  pub port_auth: u16,
  pub port_room_management: u16,
}

impl Config {
  pub fn from_env() -> Self {
    envy::from_env().expect("Failed to load environment variables")
  }
}
