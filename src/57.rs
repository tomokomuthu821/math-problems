// Implement a function to calculate the factorial of a number.
fn factorial(n: u32) -> u32 {
    if n == 0 || n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Implement a function to convert a string to an integer.
fn str_to_int(s: &str) -> Option<u32> {
    match u32::from_str(s) {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}
