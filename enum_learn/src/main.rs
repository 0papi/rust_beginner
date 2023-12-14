
use std::option::Option;



mod garden {

}

fn main() {
    // declaring enums
    #[derive(Debug)]
    enum IPAddressKind {
        V4,
        V6
    }

    // creating instances of enums
    // when create the instances,
    // each instance has the type of the enum. thus version4 below has type IPAddressKind
    let version4 = IPAddressKind::V4;
    let version6 = IPAddressKind::V6;

    // we can use this to do anything regarding ip addresses
    // while also using the enum declaration as a type
    fn router(ip_kind: IPAddressKind) {
        println!("Do something with it, {:?}", ip_kind);
    }

    // the above declaration of enum is only being used to define the kinds of IP Addresses
    // in the below code, we define a new enum which directly has value associations so that we dont
    // for instance use structs to capture the data
    enum IPAddrKind {
        V4(String),
        V6(String)
    }

    // when we use the enum above, we can bind values to it like so
    let home = IPAddrKind::V4(String::from("127.0.0.0"));
    let loopback = IPAddrKind::V6(String::from("::1"));


    // we can create an enum and bind to it multiple values
    #[derive(Debug)]
    enum Message {
        Quit,
        Write(String),
        Move {x: i32, y:i32},
        ChangeColor (i32, i32, i32)
    }

    // attach methods to the enum
    impl Message {
        fn call(&self){
            // carry out actions here
        }
    }

    // the enum above has four different variants which can be used to undertake various actions;
    // example below
    let message = Message::Write(String::from("Hello there"));

    // call method added to enum should exist on message
    message.call(); // method added to enum

    println!("Enum testing: {:?}", message);

    // Writing Methods to Enums
    // Like structs, we can also attach methods to enums & just like struct, it uses the same keyword impl
    // example below

    // testing();

    let pesewa_description = description_of_value(Denominations::Pesewas);
    let cedi_description = description_of_value(Denominations::Cedis);

    println!("{}", pesewa_description);
    println!("{}", cedi_description);


    // calling the option enum here
    let result = divide(0.0, 20.4);
    match result {
        Some(value) => println!("Result: {}", value),
        None  => println!("Cannot divide by zero!")
    }


    // testng the catch all
    catch_all_test(9);
    catch_all_test(3);
    catch_all_test(7);
    catch_all_test(1);


    // using if let instead of match
    // in the code below we are able to cater to just one case of the option enum instead of having to match all possible
    // cases as is the requirement when using match
    // if let provides a nice way to handle the Some case if that is all we are interested in without worrying about the None case
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

}



// Option as Enum
// option is Rust's way of introducing null checkers or null values without explicitly providing a Null type as is
// common in many programming language. The Option enum is so useful it is part of Rust's lib and it is define as below

//
// fn testing(){
//     let x: i8 = 5;
//     let y: Option<i8> = Some(5);
//
//     // this code will error because i8 is not the same as Option<i8>
//     let add = x + y;
//     // println!("Value added = {}", add);
// }

// MATCHING
// Matching is a powerful tool in Rust that allows us to compare a value against a series of cases or patterns
// and execute code that fits each pattern. In JavaScript, this could be a switch statement where each case
// performs a certain action so that when that case is active, the action block is executed.

// in the example below we use pattern matching to check variants of the Ghana currency and return certain string descriptions

enum Denominations {
    Pesewas,
    Cedis
}

fn description_of_value(val: Denominations) -> String {
    match val {
        Denominations::Pesewas => String::from("This is a pesewa"),
        Denominations::Cedis => String::from("This is a cedi")
    }
}


// an example of the option enum
// this function will return an Option Enum so where it is called the two cases must be handled
fn divide(x:f64, y:f64) -> Option<f64> {
    if x == 0.0 {
        None
    } else {
        let calc: f64 = x / y;
        Some(calc)
    }
}

// Catch-All & _ Placeholder
// when dealing with enums rust also allows us to not match all the patterns or variants and instead
// match the ones we want and implement a catch all for all other patterns or cases or variants
// remember that in the catch all if we do not have any use for the other cases, we can use _ instead of using
// other
fn catch_all_test(x: i32){
    match x {
        3 => println!("You rolled 3"),
        7 => print!("You rolled 7"),
        9 => println!("Hooray you rolled the right one"),
        other => println!("Oh no what you rolled is trash, {}", other)
    }
}
