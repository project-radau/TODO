#![allow(dead_code)]

use super::todo::Todo;
use crate::infrastructure::todo_repository::TodoRepositoryTrait;

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum TodoError {
    NotFound,
    Database(sqlx::Error)
}

impl Display for TodoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoError::NotFound => write!(f, "Todo not found."),
            TodoError::Database(error) => write!(f, "Database error: {}", error)
        }
    }
}

pub struct TodoService<R> where R: TodoRepositoryTrait {
    repository: R,
}

impl<R> TodoService<R> where R: TodoRepositoryTrait {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_all(&self) -> Result<Vec<Todo>, TodoError> {
        self.repository.get_all().await
    }

    pub async fn add_todo(&self, title: &str) -> Result<i64, TodoError> {
        let todo = Todo::new(0, title);

        self.repository.insert(todo).await
    }

    pub async fn remove_todo(&self, id: i64) -> Result<Todo, TodoError> {
        self.repository.delete(id).await
    }

    pub async fn complete_todo(&self, id: i64) -> Result<(), TodoError> {
        let mut todo = self.repository.get(id).await?;

        todo.complete();

        self.repository.update(todo).await
    }

    pub async fn edit_todo(&self, id: i64, title: &str) -> Result<(), TodoError> {
        let mut todo = self.repository.get(id).await?;

        todo.set_title(title);

        self.repository.update(todo).await
    }

    pub async fn find_todo(&self, id: i64) -> Result<Todo, TodoError> {
        self.repository.get(id).await
    }
}