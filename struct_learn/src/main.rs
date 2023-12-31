
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

// adding a method to the struct
impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

#[derive(Debug)]
struct User {
    name:   String,
    active: bool,
    email:  String,
    sign_in_count: u32
}

impl User {
    fn user_info(&self) {
        println!("This user's name is {}", self.name)
    }
}

#[derive(Debug)]
struct Color (i32, i32, i32);

fn main() {
    println!("Hello, world!");

    let first_user  = User {
        name: String::from("Evans Musk"),
        active: false,
        email: String::from("user@example.com"),
        sign_in_count: 2
    };

    println!("First struct:{:?}", first_user);

    // changing or mutating values in a struct
    let mut second_user = User {
        sign_in_count: 3,
        email: String::from("book@needshelp.com"),
        name: String::from("ElonMusketeer"),
        active: true
    };

    println!("BEFORE: {:?}", second_user);

    second_user.user_info();

    second_user.email = String::from("changedbookshep@help.com");

    println!("AFTER: {:?}", second_user);


    // tuple structs
    let main = Color(0, 0, 0);

    println!("Tuple Struct Ex: {:?}", main);


    let dimensions = Rectangle {
        width: 30,
        height: 50
    };

    println!("RECTANGE DIMENSIONS: {:#?}", dimensions);

    println!("RECT AREA USING METHOD = {} pixels", dimensions.area());


    // let area = rectangle_calculator(&dimensions);
    //
    // println!("AREA COMPUTED FOR RECTANGLE = {} pixels", area);
}

fn rectangle_calculator(dimensions: &Rectangle) -> i32 {
    dimensions.height * dimensions.width
}
