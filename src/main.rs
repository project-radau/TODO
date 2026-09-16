mod application;
use application::todo::Todo;

fn main() {

    let mut todos: Vec<Todo> = Vec::new();


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

    for todo in &todos {
        todo.print();
    }

    println!("{}", todos.len());
}
