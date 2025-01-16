fn chocolate_feast(n: i32, c: i32, m: i32) -> i32 {
    // Initial chocolates Bobby can buy
    let mut chocolates = n / c;
    let mut wrappers = chocolates;

    // Get free chocolates from wrappers
    while wrappers >= m {
        let free_chocolates = wrappers / m;
        chocolates += free_chocolates;
        wrappers = free_chocolates + (wrappers % m); // Update wrappers
    }

    chocolates
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read number of test cases
    let t: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Process each test case
    for _ in 0..t {
        let input: Vec<i32> = lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let n = input[0];
        let c = input[1];
        let m = input[2];

        // Calculate and print the result for this test case
        println!("{}", chocolate_feast(n, c, m));
    }
}
