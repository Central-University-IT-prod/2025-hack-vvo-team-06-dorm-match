use dormmatch_common::config::env::Config;
use sqlx::PgPool;

pub async fn init_db(config: &Config) -> PgPool {
  PgPool::connect(&config.database_url)
    .await
    .expect("Failed to connect to PostgreSQL")
}
