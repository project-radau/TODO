use super::todo::Todo;

pub fn initialize_todos() -> Vec<Todo> {

    let mut todos = Vec::new();

    todos.push(Todo::new(1, "Learn Rust"));
    todos.push(Todo::new(2, "Learn Rust 2"));

    todos
}

pub fn add_todo(todos: &mut Vec<Todo>, title: &str) {
    todos.push(Todo::new(todos.len() as u32 + 1, title));
}

pub fn remove_todo(todos: &mut Vec<Todo>, id: u32) -> Result<(), String> {
    for (index, todo) in todos.iter().enumerate() {
        if todo.id == id {
            todos.remove(index);
            return Ok(());
        }
    }

    Err(String::from("todo not found"))
}

pub fn find_todo_mut(todos: &mut [Todo], id: u32) -> Result<&mut Todo, String> {
    for todo in todos {
        if todo.id == id {
            return Ok(todo);
        }
    }

    return Err(String::from("todo not found"));
}

pub fn find_todo(todos: &[Todo], id: u32) -> Option<&Todo> {
    for todo in todos {
        if todo.id == id {
            return Some(todo);
        }
    }

    return None;
}