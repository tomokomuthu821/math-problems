// Example Rust code

fn main() {
    // Code that generates random numbers between 1 and 100
    let mut numbers = vec![];
    loop {
        println!("Guess a number:");
        match std::io::stdin().read_line(&mut numbers).expect("Failed to read line") {
            Ok(_) => break,
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    // Function that generates random triangles
    fn generate_triangle() -> (i32, i32, i32) {
        let mut side1 = rand::thread_rng().gen_range(1, 10);
        let mut side2 = rand::thread_rng().gen_range(1, 10);
        if side1 > side2 {
            let temp = side1;
            side1 = side2;
            side2 = temp;
        }

        (side1, side2, side1 + side2)
    }

    // Function that generates random squares
    fn generate_square(side: i32) -> (i32, i32) {
        let square = side * side;
        if side > 0 {
            return (square / side as f64, square / side as f64);
        }
        (1, 0)
    }

    // Main function to play with random numbers and triangles/squares
    println!("Random Triangles: {:?}", generate_triangle());
    println!("Random Squares: {:?}", generate_square(5));
}
