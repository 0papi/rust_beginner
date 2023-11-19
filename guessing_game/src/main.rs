use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to guess a number!");

    let random_number = rand::thread_rng().gen_range(1..=100);

    println!("Please enter a number: ");

    loop {
        let mut guess_number = String::new();

    io::stdin()
    .read_line(&mut guess_number).expect("Error reading value");

   
    //    convert string to integer
    let guess_number: i32 = match guess_number.trim().parse() {
        Ok(number) => number,
        Err(_) => continue
    };

    println!("Your guess: {}", guess_number);

    match guess_number.cmp(&random_number) {
        Ordering::Less => println!("Your guess is too low!"),
        Ordering::Greater => println!("Your guess is too high!"),
        Ordering::Equal => {
            println!("Your guess is correct!");
            break;
        }
    }
    }
}
