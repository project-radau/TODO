#![allow(dead_code)]

use crate::application::{todo_service::TodoError};

use sqlx::{SqlitePool};

use crate::application::todo::Todo;

pub struct TodoRepository {
    database: SqlitePool,
}

struct TodoRow {
    id: i64,
    title: String,
    completed: i64,
}

impl TodoRepository {
    pub fn new(database: SqlitePool) -> Self {
        Self { database }
    }

    pub async fn get_all(&self) -> Result<Vec<Todo>, sqlx::Error> {
        let rows = sqlx::query_as!(
            TodoRow,
            r#"
                SELECT id, title, completed
                FROM todos
                ORDER BY id
            "#
        )
        .fetch_all(&self.database)
        .await?;

        let todos = rows
            .into_iter()
            .map(|row| {
                Todo::from_database(
                    row.id,
                    row.title,
                    row.completed != 0,
                )
            })
            .collect();

        Ok(todos)
    }

    pub async fn get(&self, id: i64) -> Result<Todo, TodoError> {
        let row = sqlx::query_as!(
            TodoRow,
            r#"
                SELECT id, title, completed
                FROM todos
                WHERE id = ?
                LIMIT 1
            "#,
            id
        ).fetch_optional(&self.database)
        .await
        .map_err(|_| TodoError::NotFound)?
        .ok_or(TodoError::NotFound)?;

        Ok(Todo::from_database(row.id, row.title, row.completed != 0))
    }

    pub async fn delete(&self, id: i64) -> Result<Todo, TodoError> {
        let todo = self.get(id).await?;

        let result = sqlx::query!(
            r#"
                DELETE FROM todos
                WHERE id = ?
            "#,
            id
        )
        .execute(&self.database)
        .await
        .map_err(|_| TodoError::NotFound)?;

        if result.rows_affected() == 0 {
            return Err(TodoError::NotFound);
        }

        Ok(todo)
    }

    pub async fn update(&self, todo: Todo) -> Result<(), TodoError> {
        let result = sqlx::query!(
            r#"
                UPDATE todos
                SET title = ?, completed = ?
                WHERE id = ?
            "#,
            todo.title(),
            todo.completed(),
            todo.id()
        ).execute(&self.database)
        .await
        .map_err(|_| TodoError::NotFound)?;

        if result.rows_affected() == 0 {
            return Err(TodoError::NotFound);
        }

        Ok(())
    }

    pub async fn insert(&self, todo: Todo) -> Result<(), TodoError> {
        sqlx::query!(
            r#"
                INSERT INTO todos (title, completed)
                VALUES (?, ?)
            "#,
            todo.title(),
            todo.completed()
        ).execute(&self.database)
        .await
        .map_err(|_| TodoError::NotFound)?;

        Ok(())
    }
}