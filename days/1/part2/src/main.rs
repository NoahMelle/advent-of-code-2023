use std::fs;

fn main() {
    let input = parse_input("./src/input.txt");

    let mut total: u64 = 0;

    let number_words = vec![
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    for line in input {
        let chars: Vec<char> = line.chars().collect();
        let mut digits: Vec<char> = Vec::new();

        let mut i = 0;
        while i < chars.len() {
            let mut matched = false;

            for (word, digit_char) in &number_words {
                if i + word.len() <= chars.len() {
                    let slice: String = chars[i..i + word.len()].iter().collect();

                    if slice == *word {
                        digits.push(*digit_char);

                        matched = true;

                        break;
                    }
                }
            }

            if !matched {
                if chars[i].is_numeric() {
                    digits.push(chars[i]);
                }
            }

            i += 1;
        }

        if let (Some(first), Some(last)) = (digits.first(), digits.last()) {
            let combined = format!("{}{}", first, last);
            if let Ok(value) = combined.parse::<u64>() {
                total += value;
            }
        }
    }

    println!("Total: {total}");
}

fn parse_input(filename: &str) -> Vec<String> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines().map(|line| line.to_string()).collect()
}
