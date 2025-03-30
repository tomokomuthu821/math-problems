fn main() {
    // Generate random numbers between 1 and 100
    let random_numbers: Vec<i32> = (1..=100).collect();
    
    // Shuffle the vector of random numbers to make it more difficult
    let mut shuffled_numbers = random_numbers;
    for i in 0..shuffled_numbers.len() {
        // Randomly select an index between 0 and len-1
        let j = rand::random::<usize>();
        
        // Swap the current element with the randomly selected one
        std::mem::swap(&mut shuffled_numbers[i], &mut shuffled_numbers[j]);
    }
    
    // Print the generated random numbers
    println!("Random Numbers: {:?}", shuffled_numbers);
}
