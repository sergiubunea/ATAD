use std::collections::HashMap;

fn sherlock_and_anagrams(s: &str) -> u32 {
    let mut pair_count = 0;
    let mut substring_map = HashMap::new();

    // Generate all substrings
    for i in 0..s.len() {
        for j in i + 1..=s.len() {
            // Get the substring from i to j
            let substring = &s[i..j];
            // Sort the substring and use it as a key
            let mut sorted_substring: Vec<char> = substring.chars().collect();
            sorted_substring.sort();
            let sorted_substring: String = sorted_substring.iter().collect();
            
            // If this sorted substring has appeared before, add its frequency to pair count
            if let Some(&count) = substring_map.get(&sorted_substring) {
                pair_count += count;
            }

            // Increment the frequency of this sorted substring in the map
            *substring_map.entry(sorted_substring).or_insert(0) += 1;
        }
    }

    pair_count
}

fn main() {
    // Read number of queries
    let mut q_input = String::new();
    std::io::stdin().read_line(&mut q_input).unwrap();
    let q: u32 = q_input.trim().parse().unwrap();

    // Process each query
    for _ in 0..q {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let s = s.trim();
        
        // Calculate and print the result for the current string
        println!("{}", sherlock_and_anagrams(s));
    }
}
