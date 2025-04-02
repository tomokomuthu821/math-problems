use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().expect("Please enter a positive integer");

    for i in 1..=n {
        if i % 2 == 0 {
            println!("{}", "Fizz");
        } else if i % 3 == 0 {
            println!("{}", "Buzz");
        } else {
            print!("{} ", i);
        }
    }
}
