use std::ops;

fn main() {
    let x = 5;
    let y = 3;

    println!("The result of {} + {} is {}", x, y, ops::Add::add(x, y));
}
