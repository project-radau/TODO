mod application;

use application::todo::Todo;
use application::todo_service::{
    add_todo,
    find_todo,
    find_todo_mut,
    initialize_todos,
    remove_todo,
};

fn print_todos(todos: &[Todo]) {
    for todo in todos {
        todo.print();
    }
}

fn main() {
    let mut todos = initialize_todos();

    print_todos(&todos);

    add_todo(&mut todos, "Learn Tauri");

    print_todos(&todos);

    match remove_todo(&mut todos, 3) {
        Ok(()) => println!("Todo removed"),
        Err(error) => println!("{}", error),
    }

    match find_todo(&todos, 2) {
        Some(todo) => todo.print(),
        None => println!("Todo not found"),
    }

    match find_todo_mut(&mut todos, 2) {
        Ok(todo) => todo.complete(),
        Err(error) => println!("{}", error),
    }

    print_todos(&todos);
}