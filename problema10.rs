fn diagonal_difference(arr: Vec<Vec<i32>>) -> i32 {
    let mut left_to_right_sum = 0;
    let mut right_to_left_sum = 0;
    let n = arr.len();

    for i in 0..n {
        left_to_right_sum += arr[i][i]; // Element on the left-to-right diagonal
        right_to_left_sum += arr[i][n - 1 - i]; // Element on the right-to-left diagonal
    }

    (left_to_right_sum - right_to_left_sum).abs() // Return the absolute difference
}

fn main() {
    use std::io;

    let mut input = String::new();

    // Read the size of the matrix (n)
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid number");

    let mut matrix = Vec::new();

    // Read the matrix
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let row: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid number"))
            .collect();
        matrix.push(row);
    }

    // Calculate and print the result
    let result = diagonal_difference(matrix);
    println!("{}", result);
}
