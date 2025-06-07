use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};


/// This module defines the data model for the todo application.
/// It includes the `Todo` struct which represents a todo item in the database.

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub created_at: NaiveDateTime
}

