use std::io;

fn plus_minus(arr: Vec<i32>) {
    let total_count = arr.len() as f64;

    let positive_count = arr.iter().filter(|&&x| x > 0).count() as f64;
    let negative_count = arr.iter().filter(|&&x| x < 0).count() as f64;
    let zero_count = arr.iter().filter(|&&x| x == 0).count() as f64;

    // Calculate proportions
    let positive_ratio = positive_count / total_count;
    let negative_ratio = negative_count / total_count;
    let zero_ratio = zero_count / total_count;

    // Print results with six decimal places
    println!("{:.6}", positive_ratio);
    println!("{:.6}", negative_ratio);
    println!("{:.6}", zero_ratio);
}

fn main() {
    let mut input = String::new();

    // Read the size of the array (not directly used)
    io::stdin().read_line(&mut input).expect("Failed to read input");

    input.clear();

    // Read the array elements
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    // Call the function
    plus_minus(arr);
}
