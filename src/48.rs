// Problem: Implement a function to calculate the sum of all elements in a vector
// Example usage:
// >>> sum_of_elements(vec![1, 2, 3])
// 6
// >>> sum_of_elements(vec![])
// 0

fn sum_of_elements(numbers: Vec<isize>) -> isize {
    numbers.iter().sum()
}
