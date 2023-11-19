use std::io;

fn main() {
    // let sum = 5 + 4;
    //
    // let subs = 9484.37 - 484.484;
    //
    // // declaring a boolean in rust
    // const TEST_BOOLEAN: bool = true;
    //
    //
    // // declaring a char in rust
    // const TEST_CHAR: char = 'a';
    //
    // // declaring a tuple in rust
    // let tup: (char, char, bool) = ('Z', 'a', false );
    //
    // // using pattern matching to get values from initially declared tuple
    // let (x, _y, _z) = tup;
    //
    // // declaring an array in rust
    // // note that in rust arrays are fixed length; therefore you can annotate them like below. remember that arrays are supposed to be fixed in length therefore you cannot grow or shrink them. to use something like that which can grow or shrink, you may use Vectors in rust.
    // let testArr: [i32; 7] = [1, 3, 4, 5, 6, 7, 8];
    //
    // println!("first tupe value: {}", x);
    //
    // println!("Sum: {}", sum);
    // println!("Subs: {}", subs);
    // println!("TEST_BOOLEAN: {}", TEST_BOOLEAN);
    // println!("TEST_CHAR: {}", TEST_CHAR);

    // used this example to demo what happens when an index falls outside the limits
    // error handling will be looked at later

    let nums = [1, 2,3, 4,5];
    println!("Your numbers are: {:?}", nums);
    println!("Please select an index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Not a number");

    let element = nums[index];

    println!("The selected element is: {}", element);

}
