use std::fs;


fn main() {
    let file = fs::read_to_string("day_1.txt").expect("Unable to find file");
    println!("{}", file);
}
