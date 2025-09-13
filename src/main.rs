use std::collections::HashMap;
use std::io;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    println!("{:?}", company);

    loop {
        println!("Enter a command (Add / List / List All / Quit):");
        
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("failed to read line");
        let input = input.trim();
        
        if input.eq_ignore_ascii_case("quit") {
            println!("Goodbuye!");
            break;
        }

        println!("You typed: {}", input);

        let tokens: Vec<&str> = input.split_whitespace().collect();

        if tokens.is_empty() {continue;}

        let command = tokens[0];

        if command.eq_ignore_ascii_case("add") {
            println!("(will handle Add...)");
        } else if command.eq_ignore_ascii_case("list") {
            println!("(will handle List...)");
        } else {
            println!("Unknown command. Use: Add / List / List All / Quit");
        }
    }
}



