#![allow(dead_code)]

use crate::application::todo_service::{
    TodoService
};
use crate::input::{read_id, read_input};

fn print_menu() {
    println!();
    println!("1. List todos");
    println!("2. Add todo");
    println!("3. Complete todo");
    println!("4. Remove todo");
    println!("5. Edit todo");
    println!("6. Exit");
    println!();
}

async fn print_todos(service: &TodoService) {
    match service.get_all().await {
        Ok(todos) => {
            for todo in todos {
                println!();
                println!("{}", todo);
            }
        }
        Err(error) => println!("{}", error),
    }
}

async fn add_todo(service: &TodoService) {
    println!("Please insert the title:");
    let title = read_input();

    match service.add_todo(&title).await {
        Ok(_) => {
            println!("Todo added.");
            print_todos(service).await;
        }
        Err(error) => println!("{}", error),
    }
}

async fn complete_todo(service: &TodoService) {
    print_todos(service).await;

    println!("Which Todo should be completed? ID:");
    let input = read_id();

    match service.complete_todo(input).await {
        Ok(()) => println!("Todo completed."),
        Err(error) => println!("{}", error),
    }
}

async fn remove_todo(service: &TodoService) {
    print_todos(service).await;

    println!("Which Todo should be deleted? ID:");
    let input = read_id();

    match service.remove_todo(input).await {
        Ok(_) => println!("Todo removed."),
        Err(error) => println!("{}", error),
    }

    print_todos(service).await;
}

async fn edit_todo(service: &TodoService) {
    print_todos(service).await;

    println!("Which Todo should be edited? ID:");
    let input_id = read_id();

    match service.find_todo(input_id).await {
        Ok(_) => {
            println!("Please enter the new title:");
            let input_title = read_input();

            match service.edit_todo(input_id, input_title.trim()).await {
                Ok(()) => {
                    println!("Todo edited.");
                    print_todos(service).await;
                }
                Err(error) => println!("{}", error),
            }
        }
        Err(error) => println!("{}", error),
    }
}

async fn handle_action(choice: &str, service: &TodoService) -> bool {
    match choice {
        "1" => {
            print_todos(service).await;
            false
        }
        "2" => {
            add_todo(service).await;
            false
        }
        "3" => {
            complete_todo(service).await;
            false
        }
        "4" => {
            remove_todo(service).await;
            false
        }
        "5" => {
            edit_todo(service).await;
            false
        }
        "6" => true,
        _ => {
            println!("Unknown option.");
            false
        }
    }
}

pub async fn run(service: TodoService) {
    loop {
        print_menu();
        let input = read_input();

        let interrupt_loop = handle_action(input.as_str(), &service).await;

        if interrupt_loop {
            break;
        }
    }
}