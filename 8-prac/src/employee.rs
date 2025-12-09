use std::collections::BTreeMap;
use std::io::Write;

enum OperationError {
    MissingFields(String),
    UnknownOperation,
}
enum Operation {
    Add(String, String),
    ListDepartmentEmployees(String),
    ListAllEmployees,
    Exit,
    Err(OperationError),
}

pub fn run(organization: &mut BTreeMap<String, Vec<String>>) {
    loop {
        let operation = parse_operation(get_command());
        match operation {
            Operation::Add(name, department) => {
                // employees.insert(name, v);
                let employees = organization.entry(department).or_insert(vec![]);
                employees.push(name);
            }
            Operation::ListDepartmentEmployees(department) => {
                match organization.get(&department) {
                    Some(employees) => println!("{employees:?}"),
                    None => println!("Department does not exist"),
                };
            }
            Operation::ListAllEmployees => {
                for department in organization.keys() {
                    if let Some(employees) = organization.get(department) {
                        let mut employees = employees.clone();
                        employees.sort();
                        println!("{department}: {employees:?}");
                    }
                }
            }
            Operation::Exit => break,
            Operation::Err(error) => match error {
                OperationError::MissingFields(field) => println!("Missing field {field}"),
                OperationError::UnknownOperation => println!("Unknown Operation"),
            },
        }
    }
}

fn parse_operation(command: String) -> Operation {
    let mut split = command.split_whitespace();

    let Some(operation) = split.next() else {
        return Operation::Err(OperationError::UnknownOperation);
    };
    return match operation {
        "add" => {
            let Some(name) = split.next() else {
                return Operation::Err(OperationError::MissingFields(String::from("Name")));
            };
            split.next();
            let Some(department) = split.next() else {
                return Operation::Err(OperationError::MissingFields(String::from("Department")));
            };
            Operation::Add(String::from(name), String::from(department))
        }
        "list" => {
            if let Some(department) = split.next() {
                return match department {
                    "all" => Operation::ListAllEmployees,
                    _ => Operation::ListDepartmentEmployees(String::from(department)),
                };
            };
            Operation::ListAllEmployees
        }
        "exit" => Operation::Exit,
        _ => Operation::Err(OperationError::UnknownOperation),
    };
}

fn get_command() -> String {
    print!("> ");
    std::io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(msg) => println!("{msg}"),
    }
    input.trim().to_lowercase()
}
