use std::io;

// Define the Employee struct
struct Employee {
    employee_id: u32,
    employee_name: String,
    email: String,
    age: u32,
    phone_number: String,
}

impl Employee {
    fn new(employee_id: u32, employee_name: String, email: String, age: u32, phone_number: String) -> Self {
        Employee {
            employee_id,
            employee_name,
            email,
            age,
            phone_number,
        }
    }
}

// Function to find an employee by employee ID
fn find_employee_by_id(employee_id: u32, employees: &[Employee]) -> Option<&Employee> {
    employees.iter().find(|&emp| emp.employee_id == employee_id)
}

// Function to find employees with the same age
fn find_employees_by_age(age: u32, employees: &[Employee]) -> Vec<&Employee> {
    employees.iter().filter(|&emp| emp.age == age).collect()
}

fn main() {
    let mut employees: Vec<Employee> = Vec::new();

    loop {
        println!("Enter employee details (or type 'quit' to exit):");

        println!("Employee ID:");
        let mut employee_id = String::new();
        io::stdin().read_line(&mut employee_id).expect("Failed to read line");
        if employee_id.trim().to_lowercase() == "quit" {
            break;
        }
        let employee_id: u32 = employee_id.trim().parse().expect("Invalid input");

        println!("Employee Name:");
        let mut employee_name = String::new();
        io::stdin().read_line(&mut employee_name).expect("Failed to read line");
        let employee_name = employee_name.trim().to_string();

        println!("Email:");
        let mut email = String::new();
        io::stdin().read_line(&mut email).expect("Failed to read line");
        let email = email.trim().to_string();

        println!("Age:");
        let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Failed to read line");
        let age: u32 = age.trim().parse().expect("Invalid input");

        println!("Phone Number:");
        let mut phone_number = String::new();
        io::stdin().read_line(&mut phone_number).expect("Failed to read line");
        let phone_number = phone_number.trim().to_string();

        let employee = Employee::new(employee_id, employee_name, email, age, phone_number);
        employees.push(employee);
    }

    println!("Employee Data Collection:");
    println!("{:<15} {:<20} {:<20} {:<10} {:<15}", "ID", "Name", "Email", "Age", "Phone Number");
    for emp in &employees {
        println!("{:<15} {:<20} {:<20} {:<10} {:<15}", emp.employee_id, emp.employee_name, emp.email, emp.age, emp.phone_number);
    }

    println!("Enter an employee ID to retrieve details:");
    let mut input_id = String::new();
    io::stdin().read_line(&mut input_id).expect("Failed to read line");
    let input_id: u32 = input_id.trim().parse().expect("Invalid input");

    match find_employee_by_id(input_id, &employees) {
        Some(emp) => {
            println!("Employee Details:");
            println!("Employee ID: {}", emp.employee_id);
            println!("Employee Name: {}", emp.employee_name);
            println!("Email: {}", emp.email);
            println!("Age: {}", emp.age);
            println!("Phone Number: {}", emp.phone_number);
        }
        None => {
            println!("Employee not found.");
        }
    }

    println!("Enter an age to retrieve employees with the same age:");
    let mut input_age = String::new();
    io::stdin().read_line(&mut input_age).expect("Failed to read line");
    let input_age: u32 = input_age.trim().parse().expect("Invalid input");

    let matching_employees = find_employees_by_age(input_age, &employees);

    if !matching_employees.is_empty() {
        println!("Employees with Age {}:", input_age);
        for emp in matching_employees {
            println!("Employee Name: {}", emp.employee_name);
            println!("Email: {}", emp.email);
            println!("Phone Number: {}", emp.phone_number);
            println!("-------------------------------------");
        }
    } else {
        println!("No employees found with age {}.", input_age);
    }
}
