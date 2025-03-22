// This is an example of generating random numbers using Rust.
fn main() {
    let mut random = rand::thread_rng();
    println!("{}", random.next_u32());
}
