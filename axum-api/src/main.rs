mod models;
mod handlers; 
mod db;

use axum::{
    routing::{get, post, delete},
    Router
};

use handlers::*;
use db::get_db_pool;
use tracing_subscriber;
use dotenv::dotenv;
use std::net::SocketAddr;

// Main function to start the Axum server
// It initializes the database connection pool and sets up the routes for the todo application.
// It listens on port 3000 and serves the application using the Axum framework.
#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = get_db_pool().await;


    let app = Router::new()
    // Define the routes for the todo application
    // The routes include listing all todos, creating a new task, retrieving a task
    // by ID, and deleting a todo by ID.
    // Each route is associated with a specific handler function that processes 
    // the request and interacts with the database.
    .route("/todos", get(list_todos).post(create_todo))
    .route("/todos/{id}", get(get_todo).delete(delete_todo).put(update_todo))
    .with_state(pool);

    // Start the server and listen on port 3000
    // The server will handle incoming requests and route them to the appropriate handlers.
    // tokio tcp listener is used to bind the server to a specific address and port.
    // The server will run asynchronously, allowing it to handle multiple requests concurrently.
    // The axum::serve function is used to bind the server to a TCP listener.
    println!("Server listening on port 3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}
