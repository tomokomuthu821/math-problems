fn main() {
    // Example of using functions
    let sum = |a: i32, b: i32| -> i32 {
        a + b
    };

    let result = sum(5, 3);
    println!("The sum is: {}", result);
}
