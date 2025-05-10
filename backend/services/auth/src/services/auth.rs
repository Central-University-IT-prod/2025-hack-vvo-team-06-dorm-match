use actix_web::web;
use redis::{AsyncCommands, Client};

pub async fn store_session(
  redis_client: &web::Data<Client>,
  user_id: &str,
  token: &str,
) -> Result<(), redis::RedisError> {
  let mut conn = redis_client.get_multiplexed_async_connection().await?;
  conn
    .set_ex::<_, _, ()>(&format!("session:{}", user_id), token, 3600)
    .await?;
  Ok(())
}

pub async fn get_session(
  redis_client: &web::Data<Client>,
  user_id: &str,
) -> Result<Option<String>, redis::RedisError> {
  let mut conn = redis_client.get_multiplexed_async_connection().await?;
  conn.get(&format!("session:{}", user_id)).await
}
