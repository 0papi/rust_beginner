mod pentry;

use crate::pentry::prompt;
use crate::pentry::read_passwords_from_file;


fn clr(){

}

fn main() {
    clr();

    loop {
        println!("Password Manager Menu:");
        println!("1. Add Entry");
        println!("2. List Entry");
        println!("3. Search Entry");
        println!("4. Quit")
    }
}
