mod application;

use std::io;

use crate::application::todo_service::TodoService;

fn main() {
    let mut service = TodoService::new();

    loop {
        //actions
        println!("1. List todos");
        println!("2. Add todo");
        println!("3. Complete todo");
        println!("4. Remove todo");
        println!("5. Exit");

        //input
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let choice = input.trim();

        //match
        match input.trim() 
        { 
            "1" => 
            { 
                service.print_todos(); 
            } 
            "2" => 
            { 
                println!("Please insert the title:"); 
                let mut title = String::new(); 
                io::stdin() 
                    .read_line(&mut title) 
                    .expect("Failed to read input"); 

                service.add_todo(title.trim()); 
            } 
            "3" => 
            { 
                println!("Which Todo should be completed? ID:"); 
                let mut input = String::new(); 
                io::stdin() 
                    .read_line(&mut input) 
                    .expect("Failed to read input"); 
                
                match input.trim().parse::<u32>() 
                { 
                    Ok(id) => match service.complete_todo(id) 
                        { 
                            Ok(()) => println!("Todo completed."), 
                            Err(error) => println!("{}", error), 
                        }, 
                    Err(_) => println!("Please enter a valid ID."), 
                } 
            } 
            "4" => 
            { 
                println!("Which Todo should be deleted? ID:"); 
                let mut input = String::new(); 
                io::stdin() 
                    .read_line(&mut input) 
                    .expect("Failed to read input"); 
                
                match input.trim().parse::<u32>() 
                { 
                    Ok(id) => match service.remove_todo(id) 
                        { 
                            Ok(()) => println!("Todo removed."), 
                            Err(error) => println!("{}", error), 
                        }, 
                    Err(_) => println!("Please enter a valid ID."), 
                } 
            } 
            "5" => { break; } 
            _ => { println!("Unknown option."); } 
        }
    }
}