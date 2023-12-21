
fn main() {
    // TODO: accept inputs from user so that you can test dynamic palindromes
    let original_string = "A man, a plan, a canal, Panama!";

    if is_palindrome(original_string) {
        println!("This is a palindrome");
    } else {
        println!("Not a palindrome");
    }
}

// check reversed string against original string and return boolean value
fn is_palindrome(input: &str) -> bool {
    let sanitized_string = sanitizer(input);
    let reversed_string = reverser(&sanitized_string);

    sanitized_string == reversed_string
}

// sanitize string to make sure there are no punctuations or spaces or whitespace
fn sanitizer(input: &str) -> String {
let trimmed_string = input.trim().to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect();
    trimmed_string
}

// reverse the string
fn reverser(input: &str) -> String {
    let reversed_string = input.chars().rev().collect();
    reversed_string

}