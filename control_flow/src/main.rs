fn main() {
    show_conditions(2);
    show_conditions(0);
    show_conditions(-1);

    // using if in a let statement
    let condition = false;

    // note that when doing this you must ensure that the type in both conditions are the same. That is you cannot
    // have the if be an integer while the else is a string
    // rust will error when you do that
    let new_number = if condition { 1 } else { 3}; 

    println!("{:?}", new_number);
}


fn show_conditions(x: i32){
    if x > 0 {
        println!("x is positive");
    } else if x == 0 {
        println!("x is zero");
    } else {
        println!("x is negative");
    }
}