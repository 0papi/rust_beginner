use std::io;

// write a program that takes temp in F from user and converts it to Celsius
fn main() {
    println!("Convert F temperature to Celsius!");

    println!("Type your temperature in Fahrenheit!");

    // declare variable that will hold the new temperature value;
    // this should be mutable so that it can be updated later;
    let mut initial_temp = String::new();

    // read temp value from terminal
    io::stdin().read_line(&mut initial_temp).expect("An error occurred");

    // parse temp value as a number
    let initial_temp:  f32 = initial_temp.trim().parse().expect("An error occurred");

    // call function that'll compute the celsius and pass F temp to it;
    let temp_in_celsius = compute_temp(initial_temp);

    println!("Temperature in Celsius = {}", temp_in_celsius);
}

// this function will receive the temp value and do the conversion and then return it;
fn compute_temp(temp:f32) -> f32 {
    println!("TEMP RECEIVED= {temp}");
    let celsius_val = ((5.0 * temp) - 160.0) / 9.0;
    return celsius_val;
}
