use std::io;

// Define the Student struct
struct Student {
    id: u32,
    name: String,
    email: String,
    phno: String,
}

fn main() {
    // Create a vector to store student details
    let mut students: Vec<Student> = Vec::new();

    // Add some sample student details to the vector
    students.push(Student {
        id: 1,
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
        phno: String::from("123-456-7890"),
    });

    students.push(Student {
        id: 2,
        name: String::from("Bob"),
        email: String::from("bob@example.com"),
        phno: String::from("987-654-3210"),
    });

    students.push(Student {
        id: 3,
        name: String::from("Carol"),
        email: String::from("carol@example.com"),
        phno: String::from("555-555-5555"),
    });

    // Get student index from the user
    println!("Enter student index to retrieve details (0-{}):", students.len() - 1);
    let mut index_input = String::new();
    io::stdin().read_line(&mut index_input).expect("Failed to read line");
    let index: usize = index_input.trim().parse().expect("Invalid input");

    // Handle out-of-bounds access using match and if-let
    match students.get(index) {
        Some(student) => {
            println!("Student Details:");
            println!("ID: {}", student.id);
            println!("Name: {}", student.name);
            println!("Email: {}", student.email);
            println!("Phone Number: {}", student.phno);
        }
        None => {
            println!("Invalid student index.");
        }
    }
}
