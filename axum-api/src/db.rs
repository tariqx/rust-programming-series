use sqlx::postgres::PgPoolOptions;
use std::env;

/// This module provides the database connection pool for the todo application.
/// It initializes the connection pool using the DATABASE_URL environment variable.
pub async fn get_db_pool() -> sqlx::PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to create database pool")

}