use std::cmp;

fn main() {
    let a = 5;
    let b = 3;
    let c = 10;

    if a > b && a > c {
        println!("a is greater than both b and c");
    } else if b > a && b > c {
        println!("b is greater than both a and c");
    } else if c > a && c > b {
        println!("c is greater than both a and b");
    } else {
        println!("All conditions are not met.");
    }
}
