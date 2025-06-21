use std::fs;

fn main() {
    let input = parse_input("./src/input.txt");

    let mut total: u64 = 0;

    for line in input {
        let chars: Vec<String> = line.chars().map(|c| c.to_string()).collect();

        let mut first_number: u64 = 0;
        let mut last_number: u64 = 0;

        for char in chars {
            let parsed = char.parse::<u64>();

            match parsed {
                Ok(data) => {
                    if first_number == 0 {
                        first_number = data
                    }

                    last_number = data
                }
                Err(_) => (),
            }
        }

        let line_total = format!("{first_number}{last_number}");

        let to_add = line_total.parse::<u64>();

        if let Ok(data) = to_add {
            total += data
        }
    }

    println!("Total: {total}");
}

fn parse_input(filename: &str) -> Vec<String> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines().map(|line| line.to_string()).collect()
}
