use std::io;

fn main() {
    let mut option = String::new();

    println!("Enter your choice");
    println!("1 for Celsius to Fahrenheit");
    println!("2 for Fahrenheit to Celsius");

    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read data from standard input");

    let option: u32 = option.trim().parse().expect("Invalid Number");
    if option > 2 {
        panic!("Invalid Input");
    }
    let mut value = String::new();

    println!("Enter your value");

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read data from stdin");
    let value: f64 = value.trim().parse().expect("Invalid Number");

    let answer = match option {
        1 => convert_celsius_to_fahrenheit(value),
        2 => convert_fahrenheit_to_celsius(value),
        num => f64::from(num),
    };

    println!("Answer is {}", answer);
}

fn convert_fahrenheit_to_celsius(f_val: f64) -> f64 {
    (f_val - 32.0) / 1.8
}

fn convert_celsius_to_fahrenheit(c_val: f64) -> f64 {
    (c_val * 1.8) + 32.0
}
