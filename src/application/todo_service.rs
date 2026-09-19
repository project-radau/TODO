#![allow(dead_code)]

use super::todo::Todo;

pub struct TodoService {
    todos: Vec<Todo>,
}

impl TodoService {
    pub fn new() -> Self {
        Self {
            todos: Vec::new()
        }
    }
    
    pub fn initialize_todos(&mut self) {
        self.todos.clear();
        
        self.add_todo("Learn Rust");
        self.add_todo("Learn Tauri");
    }
    
    pub fn add_todo(&mut self, title: &str) {
        self.todos.push(Todo::new( self.todos.len() as u32 + 1, title));
    }

    pub fn remove_todo(&mut self, id: u32) -> Result<(), String> {
        for (index, todo) in self.todos.iter().enumerate() {
            if todo.id() == id {
                self.todos.remove(index);
                return Ok(());
            }
        }

        Err(String::from("todo not found"))
    }

    pub fn complete_todo(&mut self, id: u32) -> Result<(), String> {
        for todo in &mut self.todos {
            if todo.id() == id {
                todo.complete();
                return Ok(());
            }
        }

        Err(String::from("todo not found"))
    }

    pub fn edit_todo(&mut self, id: u32, title: &str) -> Result<(), String> {
        match self.find_todo_mut(id) {
            Ok(todo) => {
                todo.set_title(title);
                return Ok(());
            },
            Err(error) => return Err(error)
        };
    }

    pub fn find_todo_mut(&mut self, id: u32) -> Result<&mut Todo, String> {
        for todo in &mut self.todos {
            if todo.id() == id {
                return Ok(todo);
            }
        }

        return Err(String::from("todo not found"));
    }

    pub fn find_todo(&mut self, id: u32) -> Option<&Todo> {
        for todo in &mut self.todos {
            if todo.id() == id {
                return Some(todo);
            }
        }

        return None;
    }

    pub fn print_todos(&mut self) {
        for todo in &mut self.todos  {
            println!();
            println!("ID: {0}", todo.id());
            println!("Title: {0}", todo.title());
            println!("Complete: {0}", todo.completed());
            println!();
        }
    }
}