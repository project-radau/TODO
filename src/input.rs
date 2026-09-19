pub fn read_input() -> String {
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

pub fn read_id() -> u32 {
    loop {
        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<u32>()
        {
            Ok(id) => return id,

            Err(_) => println!("please insert a number")
        } 
    }
}