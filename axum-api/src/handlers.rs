use axum::{
    extract::{Path, State},
    Json,
    response::{IntoResponse, Response},
};
use uuid::Uuid;
use serde::Deserialize;
use sqlx::PgPool;
use crate::models::Todo;
use reqwest::{Request, StatusCode};

// This module contains the handlers for the todo application.
// It defines the functions to handle various HTTP requests related to todo items.
// Each function corresponds to a specific route and performs operations like listing, creating, retrieving, and deleting todo items.
// The handlers use Axum's extractors to get the database connection and request data.
// The CreateTodo struct is used to deserialize the request body for creating a new todo item.

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

/// list_todos
/// This function retrieves all todo items from the database.
/// It uses the SELECT SQL command to fetch all items.
/// It returns a Json Todo list containing all todo items ordered by their creation date.
pub async fn list_todos(State(pool): State<PgPool>) -> Json<Vec<Todo>> {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(todos)
}

/// create_todo
/// This function creates a new todo item in the database.
/// It uses the INSERT SQL command to add a new item.
/// It returns the created Todo item as a Json<Todo>.
/// It expects a CreateTodo struct in the request body, which contains the title of the todo item.
/// It generates a new UUID for the todo item and inserts it into the database.
pub async fn create_todo(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTodo>,
) -> Json<Todo> {
        let todo = sqlx::query_as::<_, Todo>("Insert into todos(id, title) values ($1, $2) returning *")
        .bind(Uuid::new_v4())
        .bind(payload.title)
        .fetch_one(&pool)
        .await
        .unwrap();
    Json(todo)
}

/// get_todo by id
/// This function retrieves a todo item by its ID from the database.
/// It uses the SELECT SQL command to fetch the item.
/// It returns a Json<Todo> if found, or a StatusCode::NOT_FOUND if the item does not exist.
pub async fn get_todo(Path(id): Path<Uuid>, State(pool): State<PgPool>) -> Result<Json<Todo>, StatusCode>{ 
    let todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
       .bind(id)
       .fetch_optional(&pool)
       .await
       .unwrap();

    todo.map(Json).ok_or(StatusCode::NOT_FOUND)
}

/// delete_todo
/// This function deletes a todo item by its ID and returns a simple confirmation message.
/// It uses the DELETE SQL command to remove the item from the database.
/// It returns a static string "Deleted" upon successful deletion.
pub async fn delete_todo(Path(id): Path<Uuid>, State(pool): State<PgPool>) -> &'static str {
    let _ = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await;
    "Deleted"
}

/// update_todo
/// This function updates an existing todo item by its ID.
/// It retrieves the todo item from the database, merges the fields from the request body with the existing item, and updates it.
/// It returns the updated Todo item as a Json<Todo>.
pub async fn update_todo(
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, (StatusCode, String)> {
    // Fetch existing todo
    // The SQL SELECT command is used to retrieve the existing todo item by its ID.
    // The fetch_optional method returns an Option<Todo>, which is either Some(todo) 
    // if found or None if not found.
    let existing = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "database error".into()))?;

    // Check if the todo exists
    // If the todo item does not exist, return a Not Found error.
    // The Option<Todo> type is used to handle the case where the item might not be found.
    let existing = match existing {
        Some(todo) => todo,
        None => return Err((StatusCode::NOT_FOUND, "Task not found".into())),
    };

    // Merge fields
    // If the payload has a title, use it; otherwise, keep the existing title.
    // If the payload has a completed status, use it; otherwise, keep the existing completed status.
    // This allows partial updates to the todo item.
    let new_title = payload.title.unwrap_or(existing.title);
    let new_completed = payload.completed.unwrap_or(existing.completed);

    // Update DB
    // The SQL UPDATE command is used to modify the existing todo item in the database.
    let updated = sqlx::query_as::<_, Todo>(
        "UPDATE todos SET title = $1, completed = $2 WHERE id = $3 RETURNING *",
    )
    .bind(new_title)
    .bind(new_completed)
    .bind(id)
    .fetch_one(&pool)
    .await
    // Handle errors during the update operation
    // If the update fails, return an error with a status code and message. 
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Update failed".into()))?;

    // Return the updated todo item as a JSON response
    // The Json<Todo> type is used to serialize the updated todo item into a JSON response.
    Ok(Json(updated))
}


