use std::io;

mod calc;
mod parser;

fn main() {
    println!("Enter an Equation:");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(_) => {
            println!("Invalid Equation");
            return;
        }
    }

    let expression = parser::parse(input);
    let result = expression.value();
    println!("Result: {}", result);
}
