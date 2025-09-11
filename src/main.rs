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
    }
}
