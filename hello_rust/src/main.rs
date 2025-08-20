use std::io;
mod math;

fn read_int(prompt: &str) -> i32 {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().parse().expect("Not a valid number!")
}

fn main() {
    println!("WELCOME USER!!");


    let number1 = read_int("Enter first number:");
    let number2 = read_int("Enter second number:");

    println!("You entered: {} and {}", number1, number2);

    let res = math::add(number1,number2);
    println!("sum is:{}", res);

    let res = math::subtraction(number1,number2);
    println!("subtraction is:{}", res);

    let res = math::multiplication(number1,number2);
    println!("Multiplication is: {}", res);
    
    let res = math::value_of_pi();
    println!("Value of pi is: {}", res);
    
}
