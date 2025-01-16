use std::io;

fn mini_max_sum(arr: Vec<u64>) {
    let total_sum: u64 = arr.iter().sum();
    let min_value = arr.iter().min().unwrap();
    let max_value = arr.iter().max().unwrap();

    // Calculate the minimum sum by excluding the maximum value
    // Calculate the maximum sum by excluding the minimum value
    let min_sum = total_sum - max_value;
    let max_sum = total_sum - min_value;

    // Print the results
    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let mut input = String::new();

    // Read input line
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let arr: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    // Call the function
    mini_max_sum(arr);
}
