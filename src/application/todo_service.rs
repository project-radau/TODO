#![allow(dead_code)]

use super::todo::Todo;
use std::fmt::{Display, Formatter};

pub enum TodoError {
    NotFound,
}

impl Display for TodoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoError::NotFound => write!(f, "Todo not found."),
        }
    }
}

pub struct TodoService {
    todos: Vec<Todo>,
    next_id: u32,
}

impl TodoService {
    pub fn new() -> Self {
        Self {
            todos: Vec::new(),
            next_id: 0
        }
    }
    
    pub fn initialize_todos(&mut self) {
        self.todos.clear();
        self.next_id = 0;
        
        self.add_todo("Learn Rust");
        self.add_todo("Learn Tauri");
    }
    
    pub fn add_todo(&mut self, title: &str) {
        self.todos.push(Todo::new(self.next_id, title));
        self.next_id += 1;
    }

    pub fn remove_todo(&mut self, id: u32) -> Result<Todo, TodoError> {
        let index = self.todos
            .iter()
            .position(|todo| todo.id() == id)
            .ok_or(TodoError::NotFound)?;

        let item = self.todos.remove(index);
        Ok(item)
    }

    pub fn complete_todo(&mut self, id: u32) -> Result<(), TodoError> {
        self.todos
            .iter_mut()
            .find(|todo| todo.id() == id)
            .ok_or(TodoError::NotFound)?
            .complete();

        Ok(())
    }

    pub fn edit_todo(&mut self, id: u32, title: &str) -> Result<(), TodoError> {
        let todo = self.find_todo_mut(id)?;
        todo.set_title(title);

        Ok(())
    }

    pub fn find_todo_mut(&mut self, id: u32) -> Result<&mut Todo, TodoError> {
        self.todos
            .iter_mut()
            .find(|todo| todo.id() == id)
            .ok_or(TodoError::NotFound)
    }

    pub fn find_todo(&self, id: u32) -> Option<&Todo> {
        self.todos.iter().find(|todo| todo.id() == id)
    }

    pub fn print_todos(&self) {
        for todo in &self.todos  {
            println!();
            println!("ID: {0}", todo.id());
            println!("Title: {0}", todo.title());
            println!("Complete: {0}", todo.completed());
            println!();
        }
    }
}