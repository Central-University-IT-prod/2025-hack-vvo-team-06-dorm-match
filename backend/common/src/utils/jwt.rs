use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub sub: String,  // user_id
  pub role: String, // "student" or "admin"
  pub exp: usize,   // Expiration time
}

pub fn create_jwt(
  user_id: &str,
  role: &str,
  secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
  let expiration = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs()
    + 3600; // 1 hour

  let claims = Claims {
    sub: user_id.to_string(),
    role: role.to_string(),
    exp: expiration as usize,
  };

  encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(secret.as_ref()),
  )
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
  decode::<Claims>(
    token,
    &DecodingKey::from_secret(secret.as_ref()),
    &Validation::default(),
  )
  .map(|data| data.claims)
}
