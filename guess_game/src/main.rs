extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  let secret_number = rand::thread_rng().gen_range(1, 101);
  loop {
    println!("Enter a guess");
    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read from stdin");
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    // comparing the number
    match guess.cmp(&secret_number) {
      Ordering::Greater => println!("The guess is too high"),
      Ordering::Less => println!("The guess is too low"),
      Ordering::Equal => {
        println!("Correct Guess");
        break;
      }
    }
  }
}
