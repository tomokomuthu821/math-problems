use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let num1: i32 = rng.gen_range(0..50);
    let num2: i32 = rng.gen_range(0..50);
    println!("{} + {}", num1, num2);
}
