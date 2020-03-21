use std::io;
use std::collections::HashMap;

fn print_intro() {
    println!("Welcome to BigCo! Looks like you've got some departments to staff up.");
    println!("Add folks to departments by typing \"Add <name> to <department>\"");
    println!("List the employees assigned to a department by typing \"List <department>\"");
    println!("List out the whole company by typing \"List company\"");
    println!("When finished, type \"quit\"");
}

fn print_dept(company: &HashMap<String, Vec<String>>, department: &str) {
    match company.get(department) {
        Some(employees) => println!("{}: {:?}", department, employees),
        None => println!("no employees in {}", department)
    }
}

fn print_company(company: &HashMap<String, Vec<String>>) {
    for team in company.keys() {
        print_dept(company, team);
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

fn main() {
    print_intro();
    let mut employees_by_dept: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let input = get_input();
        let words: Vec<String> = input.split_whitespace().map(|s| String::from(s)).collect();
        let instruction = words.get(0).unwrap();

        if instruction == "quit" {
            println!("{:?}", employees_by_dept);
            break;
        } else if instruction == "List" {
            let department = words.get(1).expect("could not get department");
            if department == "company" {
                print_company(&employees_by_dept);
            } else {
                print_dept(&employees_by_dept, department);
            }
        } else if instruction == "Add" {
            // TODO: validate input
            let employee = words.get(1).expect("could not get employee");
            let department = words.get(3).expect("could not get department");
            let team = employees_by_dept.entry(department.to_string()).or_insert(vec![]);
            (*team).push(employee.to_string());
            (*team).sort();
            println!("Added {} to {}", employee, department);
        } else {
            println!("I don't know what you mean by '{}'", input.trim());
        }
    }
}
