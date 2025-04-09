use std::collections::HashSet;

/// Generate a set of unique random integers between 1 and 100.
fn main() {
    let mut numbers = HashSet::new();
    loop {
        let number = match rand::random::<i32>() {
            Some(n) => n,
            None => return,
        };
        if !numbers.contains(&number) {
            numbers.insert(number);
        }
    }
}
