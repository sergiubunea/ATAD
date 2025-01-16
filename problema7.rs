use std::io;

fn staircase(n: usize) {
    for i in 1..=n {
        // Print spaces and '#' symbols
        let spaces = n - i;
        let hashes = i;
        println!("{}{}", " ".repeat(spaces), "#".repeat(hashes));
    }
}

fn main() {
    let mut input = String::new();

    // Read input size
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid number");

    // Call the staircase function
    staircase(n);
}
