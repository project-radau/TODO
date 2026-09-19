#![allow(dead_code)]

use super::todo::Todo;

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

    pub fn find_todo(&self, id: u32) -> Option<&Todo> {
        for todo in &self.todos {
            if todo.id() == id {
                return Some(&todo);
            }
        }

        return None;
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