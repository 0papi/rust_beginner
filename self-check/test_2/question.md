You've been given a list of integers. The list represents the changes in frequency (positive or negative) for a device. Starting with a frequency of zero, you need to find the resulting frequency after applying all the changes in the list.

For instance, given the list [+1, -2, +3, +1], the resulting frequency would be 3, calculated as 0 + 1 - 2 + 3 + 1 = 3.

Write a function in Rust that takes in a list of integers as input and calculates the resulting frequency after applying all the changes.

For an extra challenge, if the list of frequency changes is provided repeatedly (looping), the task is to find the first frequency that appears twice.

For example, given the list [+1, -2, +3, +1, +1, -2], the resulting frequency would be 3. However, if the list is [+1, -1], the resulting frequency would first reach 0 twice. So, the answer for this list would be 0.


# Question 2
Write a function in Rust that performs basic string compression using the counts of repeated characters. For example, the string "aabcccccaaa" would become "a2b1c5a3". If the "compressed" string would not become smaller than the original string, your function should return the original string. You can assume the input string only contains uppercase and lowercase letters (a-z)

``fn compress_string(input: &str) -> String {
let mut compressed = String::new();
let mut count = 0;

    for (i, current_char) in input.chars().enumerate() {
        count += 1;

        // Check if the next character is different or if it's the end of the string
        if i + 1 >= input.len() || current_char != input.chars().nth(i + 1).unwrap() {
            compressed.push(current_char);
            compressed.push_str(&count.to_string());
            count = 0;
        }
    }

    if compressed.len() < input.len() {
        compressed
    } else {
        input.to_string()
    }
}
`