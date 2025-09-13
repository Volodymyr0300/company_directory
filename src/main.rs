use std::collections::HashMap;
use std::io;

fn parse_add_command(tokens: &Vec<&str>) -> Option<(String, String)> {
    let mut to_index: Option<usize> = None;
    for i in 1..tokens.len() {
        if tokens[i].eq_ignore_ascii_case("to") {
            to_index = Some(i);
            break;
        }
    }

    if let Some(i) = to_index {
        if i > 2 && i + 1 < tokens.len() {
            let name = tokens[1..i].join("");
            let department = tokens[i + 1..].join(" ");
            return Some((name, department));
        }
    }
    None
}


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

        if command.eq_ignore_ascii_case("add") {
            match parse_add_command(&tokens) {
                Some((name, department)) => {
                    println!("Parse: name '{}', department='{}'", name, department);
                }
                None => {
                    println!("Invalid Add command. Use: Add <name> to <department>");   
                }
            }
        }
    }
}



