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

fn add_employee(company: &mut HashMap<String, Vec<String>>, name: String, department: String) {
    let entry = company.entry(department).or_insert(Vec::new());
    entry.push(name);
}

fn list_department(company: &HashMap<String, Vec<String>>, department: &str) {
    match company.get(department) {
        Some(employees) => {
            let mut names = employees.clone();
            names.sort();
            println!("Department '{}':", department);
            for name in names {
                println!("- {}", name);
            }
        }
        None => {
            println!("No such department: {}", department);
        }
    }
}

fn list_all(company: &HashMap<String, Vec<String>>) {
    if company.is_empty() {
        println!("No departments / employees found.");
        return;
    }

    let mut departments: Vec<&String> = company.keys().collect();
    departments.sort();

    for dept in departments {
        println!("{}", dept);

        if let Some(employees) = company.get(dept) {
            let mut names = employees.clone();
            names.sort();
            for name in names {
                println!("- {}", name);
            }
        }
    }
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
        } else if command.eq_ignore_ascii_case("list") {
            if tokens.len() >= 2 && tokens[1].eq_ignore_ascii_case("all") && tokens.len() == 2 {
                list_all(&company);
            } else if tokens.len() >= 2 {
                let department = tokens[1..].join(" ");
                list_department(&company, &department);
            } else {
                println!("Invalid List command. Use: List <department> or List All");
            }
            
        }

        if command.eq_ignore_ascii_case("add") {
            match parse_add_command(&tokens) {
                Some((name, department)) => {
                    add_employee(&mut company,name.clone(), department.clone());
                    println!("Added '{}' to '{}'.", name, department);
                }
                None => {
                    println!("Invalid Add command. Use: Add <name> to <department>");
                }
            }
        }
    }
}



