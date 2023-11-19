fn main() {
    const NUMBER_OF_TRIES : u32 = 100; // in rust this is called a constant and it is a variable that will never be changed in the lifetime of the program. Constants can never be reassigned.

    println!("You are allowed to try: {} times", NUMBER_OF_TRIES);
   let mut x = 6;

   println!("Your value is: {}", x);

   x = 12;

   println!("Your value is: {}", x);

    //    shadowing variables

    let x = 5;

    // here the previous variable x is being shadowed or overwritten by the second variable but this is not a mutation as the previous variable x's value is still in tact and being used by the second variable.
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");



}
