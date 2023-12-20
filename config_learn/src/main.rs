use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let new_veggie = Asparagus {
        count: 23,
        title: String::from("Asparagus")
    };


    println!("Hello, world!, {:?}", new_veggie);
}
