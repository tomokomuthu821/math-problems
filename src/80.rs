// Problem 1: Simple Addition
fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}

// Function to check if two numbers are equal
fn is_equal(a: i32, b: i32) -> bool {
    a == b
}

// Problem 2: Find the largest number in an array
fn find_largest_number(numbers: Vec<i32>) -> i32 {
    let mut max_num = numbers[0];
    for &num in numbers.iter() {
        if num > max_num {
            max_num = num;
        }
    }
    max_num
}

// Problem 3: Generate a random number between two values
fn generate_random_number(min: i32, max: i32) -> i32 {
    let range = max - min + 1;
    (min..=max).choose_range(range)
}

// Function to determine if a string contains only alphabetic characters
fn is_string_alphabetic(s: &str) -> bool {
    s.chars().all(|c| c.is_alpha())
}
