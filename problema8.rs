use std::io;

fn divisible_sum_pairs(n: usize, k: i32, ar: Vec<i32>) -> i32 {
    let mut count = 0;

    for i in 0..n {
        for j in i + 1..n {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let mut input = String::new();

    // Read the first line for n and k
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let params: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();
    let n = params[0];
    let k = params[1] as i32;

    input.clear();

    // Read the second line for the array elements
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let ar: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    // Call the function
    let result = divisible_sum_pairs(n, k, ar);

    // Print the result
    println!("{}", result);
}
