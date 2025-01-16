use std::io;

fn birthday_cake_candles(candles: Vec<i32>) -> i32 {
    // Find the tallest candle
    let max_height = candles.iter().max().unwrap();
    
    // Count how many candles have the tallest height
    candles.iter().filter(|&&height| height == *max_height).count() as i32
}

fn main() {
    let mut input = String::new();

    // Read the size of the array (not directly used)
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    input.clear();

    // Read the candle heights
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let candles: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid number"))
        .collect();

    // Get the result
    let result = birthday_cake_candles(candles);

    // Print the result
    println!("{}", result);
}
