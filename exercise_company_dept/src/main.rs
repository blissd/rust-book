use std::{borrow::BorrowMut, collections::HashMap};

fn main() {
    println!("Company department and worker manager.");
    println!("Enter commands like:");
    println!("Add <person> to <department>");
    println!("Remove <person> from <department>");
    println!("Show <department|all>");
    println!();

    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Enter command: ");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed reading string");

        let words: Vec<&str> = input.split_whitespace().collect();
        let command = words[0].to_lowercase();

        if command == "add" {
            let person_name = words[1];
            let department_name = words[3];
            let department = departments
                .entry(department_name.to_string())
                .or_insert(Vec::new());
            department.push(person_name.to_string());
        } else if command == "remove" {
            let person_name = words[1];
            let department_name = words[3].to_string();

            // What a pain to remove an element!
            // Surely there is a better way?
            departments
                .entry(department_name.to_string())
                .and_modify(|d| {
                    if let Some(index) = d.iter().position(|p| p == person_name) {
                        d.remove(index);
                    }
                });
        } else if command == "show" {
            let department_name = words[1].to_string();

            let mut people = if let Some(people) = departments.get(&department_name) {
                people.clone()
            } else if department_name == "all" {
                let mut people = Vec::new();
                // I wanted to use departments.values().collect()... but couldn't get it to compile :-(
                for person in departments.values().flatten() {
                    people.push(person.clone());
                }
                people
            } else {
                let empty = Vec::new();
                empty
            };

            people.sort();

            println!("Department {department_name} has the following members:");
            for person in people {
                println!("{}", person);
            }
        }
    }
}
