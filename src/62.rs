fn main() {
    // Example of using Rust's `HashMap` to store numbers.
    let mut numbers = HashMap::new();

    // Adding some numbers to the map.
    numbers.insert(10);
    numbers.insert(20);
    numbers.insert(30);

    // Accessing an existing key and getting its value.
    let x = 20;
    println!("The value associated with key {} is {}", x, numbers.get(&x).unwrap_or(&"Key not found".to_string()));

    // Removing a key-value pair from the map.
    numbers.remove(&30);

    // Iterating over and printing all values in the map.
    for (key, value) in numbers.iter() {
        println!("The value associated with {} is {}", key, value);
    }
}
