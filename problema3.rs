use std::io;

fn compare_triplets(a: Vec<i32>, b: Vec<i32>) -> (i32, i32) {
    let mut alice_score = 0;
    let mut bob_score = 0;

    for i in 0..3 {
        if a[i] > b[i] {
            alice_score += 1;
        } else if a[i] < b[i] {
            bob_score += 1;
        }
    }

    (alice_score, bob_score)
}

fn main() {
    let mut input = String::new();

    // Read Alice's ratings
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let a: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    input.clear();

    // Read Bob's ratings
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let b: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    // Compare triplets and get the scores
    let (alice_score, bob_score) = compare_triplets(a, b);

    // Print the scores
    println!("{} {}", alice_score, bob_score);
}
