fn time_in_words(h: i32, m: i32) -> String {
    let words = vec![
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen", "fourteen", "quarter", "sixteen", "seventeen",
        "eighteen", "nineteen", "twenty", "twenty one", "twenty two", "twenty three",
        "twenty four", "twenty five", "twenty six", "twenty seven", "twenty eight",
        "twenty nine", "half"
    ];

    match m {
        0 => format!("{} o' clock", words[h as usize]),
        1 => format!("one minute past {}", words[h as usize]),
        15 => format!("quarter past {}", words[h as usize]),
        30 => format!("half past {}", words[h as usize]),
        45 => format!("quarter to {}", words[(h % 12 + 1) as usize]),
        1..=29 => format!("{} minutes past {}", words[m as usize], words[h as usize]),
        31..=59 => format!(
            "{} minutes to {}",
            words[(60 - m) as usize],
            words[(h % 12 + 1) as usize]
        ),
        _ => unreachable!(), // This case should never be hit due to constraints.
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read hour and minute from input
    let h: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let m: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Compute and print the time in words
    println!("{}", time_in_words(h, m));
}
