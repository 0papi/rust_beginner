fn main() {
    let s = String::from("Hello");
    // cloning as done here creates a deep copy of the string initially created. In this way the s string above isnt MOVED to the second string declaration.
    let s2 = s.clone();

    takes_ownership(s2);

    // this will error because at this point s is no longer available.
    // When was passed to the takes_ownership function, it no longer became available,
    // going out of scope. Note that this is because the String type is stored n the
    // heap as opposed to the stack
    // if it were on the stack it wouldnt matter which variable has made a copy of the variable
    // in this regard the behaviour where s gets given to takes-ownership and becomes
    // invalid afterwards is known as MOVING. Remember the behaviour is similar with variables interacting with variables
    // to avoid the error that happens on the next line you can do a clone but remember that will
    // pass an entirely new copy of the s with everythhing that is stored(data) instead of just pointer, length and capacity
    println!("Logging s here to see if it throws: {}", s); // this will not error when you clone the s string instead of MOVING it


    is_referencing();

    let slice_test = String::from("FreeStyle By 21 Vybz");
    let test_one = &slice_test[..];
    let second_test = "Papiiiiiiiiiiiiii";

    let generated_first =  first_word(test_one);
    let second_generated = first_word(second_test);

    println!("Generated First: {}", generated_first);
    println!("Generated Second: {}", second_generated);

}

fn takes_ownership(s:String){
    println!("String is here: {}", s)
}



fn is_referencing(){
    // create a string that is referenced instead of MOVED
    let mut s1 = String::from("Hello yonkers");

    is_modifying_borrow(&mut s1);
    another_borrow(&mut s1);


    // this references the initial string and takes it to do whatever it wants
    // therefore the initial string is still there and can be used, it is not DROPPED or invalidated because the data isnt MOVED
    let copy_string = &s1;

    println!("Initial string: {}", s1);
    println!("String is here: {}", copy_string)
}

fn is_modifying_borrow(s: &mut String){
    // we cannot modify the string because it was borrowed. to be able to modify we have to create a mutable reference
    s.push_str(", world");
}

fn another_borrow(s: &mut String){
    s.push_str(", on drugs")
}


// this function takes a string which could be a bunch of words separated by space and returns the first
// first letter
fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();
    //     you can now loop through the bytes since they're a collection
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]
}