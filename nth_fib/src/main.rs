use std::io;

fn main() {
    // program to generate the nth digit of the fibonacci series
    // we will write this using a iterable
    // I may do a recursive version in the future

    let mut number = String::new();

    // Read the number from stdin
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read from stdin");

    // Parse the number string to integer using shadowing
    let number: u32 = number.trim().parse().expect("Invaild Input");

    println!(
        "{} position fibonacci number is {}",
        number,
        calculate_nth_fib(number)
    );
}

fn calculate_nth_fib(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    let mut c = a + b;

    if n == 1 {
        return a;
    }
    if n == 2 {
        return b;
    }

    for _ in 2..n {
        c = a + b;
        a = b;
        b = c;
    }

    c
}
