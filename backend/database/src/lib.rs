use dotenvy;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub mod agent;

pub async fn establish_connection() -> Pool<Postgres> {
    let database_url: String = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect("failed to connect to DATABASE_URL");

    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Sqlx Migration error");

    return db;
}
