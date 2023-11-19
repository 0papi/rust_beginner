fn main() {
    println!("Hello, world!");

    another_function(34);

    let new_text = String::from("This here is a new thing");

    multiple_params(20, new_text);

    return_something();
}

// passing a parameter to a function
fn another_function(a: i32){
    println!("Printing from second function");
    println!("Argument passed to this: {}", a);
}

fn multiple_params(x: i32, text: String){
    println!("We are testing something new {text} and also displaying some numbers {x}");
}

// expressions and statements
fn return_something(){
    let y = {
        let x = 1;
        x + 47
    };

    println!("We got a new value {y}")
}