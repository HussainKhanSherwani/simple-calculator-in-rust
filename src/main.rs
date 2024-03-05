use std::io;

// Define the Operation enum with variants Add, Subtract, Multiply, and Divide
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Define the calculate function to perform arithmetic operations based on the Operation enum
fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Multiply(x, y) => x * y,
        Operation::Divide(x, y) => x / y,
    }
}

fn main() {
    // Prompt the user for input
    println!("Enter the first number:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read input");
    let num1: f64 = num1.trim().parse().expect("Invalid number");

    println!("Enter the operation (+, -, *, /):");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Failed to read input");
    let operator: char = operator.trim().chars().next().expect("Invalid input");

    println!("Enter the second number:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read input");
    let num2: f64 = num2.trim().parse().expect("Invalid number");

    // Create an Operation enum instance based on user input
    let operation = match operator {
        '+' => Operation::Add(num1, num2),
        '-' => Operation::Subtract(num1, num2),
        '*' => Operation::Multiply(num1, num2),
        '/' => Operation::Divide(num1, num2),
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    // Call the calculate function with the created Operation enum instance
    let result = calculate(operation);

    // Print the result to the console
    println!("Result: {}", result);
}
