#![allow(dead_code)]

use crate::application::{todo_service::TodoError};

use sqlx::{SqlitePool};

use crate::application::todo::Todo;

#[async_trait::async_trait]
pub trait TodoRepositoryTrait {
    async fn get_all(&self) -> Result<Vec<Todo>, TodoError>;
    async fn get(&self, id: i64) -> Result<Todo, TodoError>;
    async fn delete(&self, id: i64) -> Result<Todo, TodoError>;
    async fn update(&self, todo: Todo) -> Result<(), TodoError>;
    async fn insert(&self, todo: Todo) -> Result<i64, TodoError>;
}

pub struct TodoRepository {
    database: SqlitePool,
}

struct TodoRow {
    id: i64,
    title: String,
    completed: i64,
}

#[async_trait::async_trait]
impl TodoRepositoryTrait for TodoRepository {
    async fn get_all(&self) -> Result<Vec<Todo>, TodoError> {
        let rows = sqlx::query_as!(
            TodoRow,
            r#"
                SELECT id, title, completed
                FROM todos
                ORDER BY id
            "#
        )
        .fetch_all(&self.database)
        .await
        .map_err(TodoError::Database)?;

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

    async fn get(&self, id: i64) -> Result<Todo, TodoError> {
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
        .map_err(TodoError::Database)?
        .ok_or(TodoError::NotFound)?;

        Ok(Todo::from_database(row.id, row.title, row.completed != 0))
    }

    async fn delete(&self, id: i64) -> Result<Todo, TodoError> {
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
        .map_err(TodoError::Database)?;

        if result.rows_affected() == 0 {
            return Err(TodoError::NotFound);
        }

        Ok(todo)
    }

    async fn update(&self, todo: Todo) -> Result<(), TodoError> {
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
        .map_err(TodoError::Database)?;

        if result.rows_affected() == 0 {
            return Err(TodoError::NotFound);
        }

        Ok(())
    }

    async fn insert(&self, todo: Todo) -> Result<i64, TodoError> {
        let result = sqlx::query!(
            r#"
                INSERT INTO todos (title, completed)
                VALUES (?, ?)
            "#,
            todo.title(),
            todo.completed()
        ).execute(&self.database)
        .await
        .map_err(TodoError::Database)?;

        Ok(result.last_insert_rowid())
    }
}

impl TodoRepository {
    pub fn new(database: SqlitePool) -> Self {
        Self { database }
    }
}