fn main() {
    let mut counter = 0;

    let result = loop {
        counter+= 1;


        if counter == 10 {
            break counter * 2;
        }

    };

    println!("The result is {result}");

    // loop_labels();
    // use_while();
    collection_loop();

}

fn loop_labels() {
    let mut count = 0;
    // here we are labeling the loop so that we can reference it at a later point in the code
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // here we reference the initial loop label so that we can break out of that one instead of the innermost loop as is
                // the default behaviour in rust
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn use_while(){
    let mut num = 3;

    while num != 0 {
        println!("{num}!");

        num -= 1;
    }

    println!("LIFTOFF!!!!!")
}

fn collection_loop(){
    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    // we can choose to loop through a collection using the while loop as illustrated below
    // how this is problematic:
    // 1 program is slow as each round trip must check the conditional
    // if the collection is shortened you have to update the index check otherwise the code will panic
    // a better solution is to use for loop
    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }


    // iterating through a collection using a for loop
    let new_list = [20, 40, 60, 80, 100];
    for list in new_list {
        // inside of here we get each list item and can do whatever we want
        // let's log it to the console
        println!("The current value is: {}", list);
    }
}
