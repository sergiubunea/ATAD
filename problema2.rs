use std::io;

fn simple_array_sum(ar: Vec<i32>) -> i32 {
    // Use the iterator sum method to calculate the total sum
    ar.iter().sum()
}

fn main() {
    let mut input = String::new();

    // Read the size of the array
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let _n: usize = input.trim().parse().expect("Input not a valid number");

    input.clear();
    // Read the array elements
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let ar: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    // Call the function and print the result
    let result = simple_array_sum(ar);
    println!("{}", result);
}
