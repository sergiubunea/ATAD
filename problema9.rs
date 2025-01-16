use std::collections::HashMap;

fn migratory_birds(arr: Vec<i32>) -> i32 {
    let mut frequency = HashMap::new();

    // Count the frequency of each bird type
    for &bird in &arr {
        *frequency.entry(bird).or_insert(0) += 1;
    }

    // Find the bird type with the highest frequency and lowest ID in case of ties
    let mut max_frequency = 0;
    let mut result = i32::MAX;

    for (&bird, &count) in &frequency {
        if count > max_frequency || (count == max_frequency && bird < result) {
            max_frequency = count;
            result = bird;
        }
    }

    result
}

fn main() {
    use std::io;

    let mut input = String::new();

    // Read the size of the array (not directly used)
    io::stdin().read_line(&mut input).expect("Failed to read input");

    input.clear();

    // Read the bird types
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    // Call the function and print the result
    let result = migratory_birds(arr);
    println!("{}", result);
}
