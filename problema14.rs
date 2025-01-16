fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Parse the first line: n (number of widths) and t (number of test cases)
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let t: usize = parts.next().unwrap().parse().unwrap();

    // Parse the width array
    let second_line = lines.next().unwrap().unwrap();
    let width: Vec<i32> = second_line.split_whitespace().map(|x| x.parse().unwrap()).collect();

    // Parse the test cases
    let mut results = Vec::new();
    for _ in 0..t {
        let case_line = lines.next().unwrap().unwrap();
        let mut parts = case_line.split_whitespace();
        let i: usize = parts.next().unwrap().parse().unwrap();
        let j: usize = parts.next().unwrap().parse().unwrap();

        // Find the minimum width in the range [i, j]
        let min_width = width[i..=j].iter().min().unwrap();
        results.push(min_width);
    }

    // Print the results
    for result in results {
        println!("{}", result);
    }
}
