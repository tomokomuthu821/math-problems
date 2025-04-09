fn main() {
    // Example of generating random data
    let random_int: i32 = rand::thread_rng().gen_range(10..=50);
    println!("Random integer between 10 and 50: {}", random_int);
}
