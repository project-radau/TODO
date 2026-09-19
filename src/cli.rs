#![allow(dead_code)]

use crate::application::todo_service::TodoService;
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

fn add_todo(service: &mut TodoService) {
    println!("Please insert the title:"); 
    let title = read_input();

    service.add_todo(&title); 
    service.print_todos();
}

fn complete_todo(service: &mut TodoService) {
    service.print_todos();
    println!("Which Todo should be completed? ID:"); 
    let input = read_id(); 
    
    match service.complete_todo(input) 
        { 
            Ok(()) => println!("Todo completed."), 
            Err(error) => println!("{}", error), 
        };
}

fn remove_todo(service: &mut TodoService) {
    service.print_todos();
    println!("Which Todo should be deleted? ID:"); 
    let input = read_id(); 

    match service.remove_todo(input) 
        { 
            Ok(todo) => {
                println!("Todo removed.");
                todo.print();
            }, 
            Err(error) => println!("{}", error), 
        };
    
    service.print_todos();
}

fn edit_todo(service: &mut TodoService) {
    service.print_todos();
    println!("Which Todo should be edited? ID:"); 
    let input_id = read_id();

    match service.find_todo(input_id) {
        Some(_) => {

            println!("Please enter the new title:");
            let input_title = read_input();

            match service.edit_todo(input_id, &input_title.trim()) 
                { 
                    Ok(()) => {
                        println!("Todo edited.");
                        service.print_todos();
                    }, 
                    Err(error) => println!("{}", error), 
                };
        }
        None => {
            println!("Todo not found.")
        }
    };

    
}

fn handle_action(choice: &str, service: &mut TodoService) -> bool {
    match choice
    { 
        "1" => 
        { 
            service.print_todos(); 
            false
        } 
        "2" => 
        {
            add_todo(service);
            false
        } 
        "3" => 
        { 
            complete_todo(service);
            false
        } 
        "4" => 
        { 
            remove_todo(service);
            false
        } 
        "5" => 
        {
            edit_todo(service);
            false
        }
        "6" => { true } 
        _ => 
        { 
            println!("Unknown option."); 
            false
        } 
    }
}

pub fn run() {
    let mut service = TodoService::new();
    service.initialize_todos();

    loop {
        print_menu();
        let input = read_input();

        let interrupt_loop = handle_action(input.as_str(), &mut service);
        if interrupt_loop == true {
            break;
        }
    }
}