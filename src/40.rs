fn main() {
    // Example of using an if statement
    let num = 10;
    match num % 2 {
        0 => println!("The number is even"),
        _ => println!("The number is odd"),
    }

    // Example of using a while loop with an else clause
    let i = 5;
    let mut sum = 0;
    while i > 0 {
        sum += i as i32; // Convert the integer to bytes for comparison
        i -= 1;
    }
    println!("The sum is: {}", sum);

    // Example of using a switch statement with multiple cases
    let day = "monday";
    let month = String::from("january");
    match day {
        "monday" => println!("It's Monday today."),
        "tuesday" => println!("It's Tuesday tomorrow."),
        _ => println!("Unknown day"),
    }

    // Example of using a for loop with an iterator
    let colors: Vec<&str> = vec!["red", "green", "blue"];
    for color in &colors {
        println!("{}", *color);
    }
}
