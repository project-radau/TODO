mod application;

use application::todo::Todo;

fn print_todos(todos : &Vec<Todo>) {
    for todo in todos {
        todo.print();
    }
}

fn initialize_todos() -> Vec<Todo> {

    let mut todos = Vec::new();

    todos.push(Todo {
        id: 1,
        title: String::from("Learn Rust"),
        completed: false,
    });

    todos.push(Todo {
        id: 2,
        title: String::from("Learn Rust 2"),
        completed: false,
    });

    todos
}

fn find_todo_mut(todos: &mut [Todo], id: u32) -> Result<&mut Todo, String> {
    for todo in todos {
        if todo.id == id {
            return Ok(todo);
        }
    }

    return Err(String::from("todo not found"));
}

fn find_todo(todos: &[Todo], id: u32) -> Option<&Todo> {
    for todo in todos {
        if todo.id == id {
            return Some(todo);
        }
    }

    return None;
}

fn add_todo(todos: &mut Vec<Todo>, title: &str) {
    todos.push(Todo {
        id: todos.len() as u32 + 1,
        title: String::from(title),
        completed: false
    });
}

fn remove_todo(todos: &mut Vec<Todo>, id: u32) -> Result<(), String> {
    for (index, todo) in todos.iter().enumerate() {
        if todo.id == id {
            todos.remove(index);
            return Ok(());
        }
    }

    Err(String::from("todo not found"))
}

fn main() {

    let mut todos: Vec<Todo> = initialize_todos();

    print_todos(&todos);

    add_todo(&mut todos, "Learn Tauri");

    print_todos(&todos);

    let result = remove_todo(&mut todos, 3);

    match result {
        Ok(()) => println!("Todo removed"),
        Err(error) => println!("{}", error),
    }

    print_todos(&todos);

    let found = find_todo(&todos, 4);

    let todo = found.unwrap();

    todo.print();

    
}
