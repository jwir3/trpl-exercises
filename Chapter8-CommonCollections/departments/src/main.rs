use std::io;
use std::io::Write;
use std::collections::HashMap;

fn main() {
    let mut name_to_department_map: HashMap<String, String> = HashMap::new();

    loop {
        print!("Please enter a command ('Quit' to end): ");

        io::stdout().flush().expect("Unable to flush output");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read command");

        let should_quit = process_command(input, &mut name_to_department_map);
        if should_quit {
            break;
        }
    }
}

fn process_command(input: String, name_to_department_map: &mut HashMap<String, String>) -> bool {
    let input_lowercase = input.to_ascii_lowercase();
    let iter = input_lowercase.split_whitespace();

    let entries: Vec<&str> = iter.collect();
    if entries[0] == "quit" {
        true
    } else if entries[0] == "add" {
        let name = String::from(entries[1]);
        let dept;
        if entries.len() == 4 {
            dept = String::from(entries[3]);
        } else {
            dept = String::from(entries[2]);
        }

        name_to_department_map.insert(name, dept);
        false
    } else {
        // Show command was seen - we can view either 'company' or specific department
        if entries[1] == "company" {
            let depts = get_sorted_list_of_department_names(name_to_department_map);
            for department in depts {
                display_all_entries_for_department(name_to_department_map, department);
            }
            false
        } else {
            let dept = entries[1];
            display_all_entries_for_department(name_to_department_map, dept.to_string());
            false
        }
    }
}

fn get_sorted_names_for_department(map: &HashMap<String, String>, dept: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for (name, department) in map {
        let lc_department = department.to_lowercase();
        let lc_dept = dept.to_lowercase();
        if &lc_department[..] == &lc_dept[..] {
            result.push(to_capitalized(&name[..]));
        }
    }

    result.sort_unstable();
    result
}

fn to_capitalized(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn get_sorted_list_of_department_names(map: &HashMap<String, String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut cap_result: Vec<String> = Vec::new();
    for val in map.values() {
        if !result.contains(val) {
            result.push(val.to_string());
        }
    }

    for next_uncapitalized in result {
        cap_result.push(to_capitalized(&next_uncapitalized));
    }

    cap_result.sort_unstable();
    cap_result
}

fn display_all_entries_for_department(
    name_to_department_map: &HashMap<String, String>,
    dept: String,
) {
    println!("");
    println!("{}", to_capitalized(&dept[..]));
    println!("----------");
    let names = get_sorted_names_for_department(name_to_department_map, dept.to_string());
    for name in names {
        println!("{}", name);
    }

    println!("");
}
