use crate::application::todo_service::TodoService;
use crate::input::{read_id, read_input};

fn print_menu() {
    println!();
    println!("1. List todos");
    println!("2. Add todo");
    println!("3. Complete todo");
    println!("4. Remove todo");
    println!("5. Exit");
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
            Ok(()) => println!("Todo removed."), 
            Err(error) => println!("{}", error), 
        };
    
    service.print_todos();
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
        "5" => { true } 
        _ => 
        { 
            println!("Unknown option."); 
            false
        } 
    }
}

pub fn run() {
    let mut service = TodoService::new();

    loop {
        //actions
        print_menu();
        //input
        let input = read_input();

        //match
        let interrupt_loop = handle_action(input.as_str(), &mut service);
        if interrupt_loop == true {
            break;
        }
    }
}