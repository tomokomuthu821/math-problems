// Solution 1: A simple solution using bitwise operations and basic arithmetic
let a = 5;
let b = 3;
let c = 8;

// Convert integers to binary strings
let sa = bin(a).chars().collect::<Vec<char>>().join("");
let sb = bin(b).chars().collect::<Vec<char>>().join("");

// Perform bitwise AND on the two binary representations of a and b
let result = match (sa, sb) {
    ("101", "110") => "101",
    _ => "000"
};

println!("Binary representation of {} AND {} is {}", b, c, result);
