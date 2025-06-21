use std::{collections::HashMap, fs};

fn main() {
    let max_cubes_per_color = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let input = parse_input("./src/input.txt");

    let mut total: u64 = 0;

    for game in input {
        let mut valid = true;

        let line = &game[5..];

        let parts: Vec<&str> = line.split(": ").collect();
        let (id, line) = (parts[0], parts[1]);

        let id: u64 = str::parse::<u64>(id).expect("Failed to parse ID");

        let sets = line.split("; ");

        for set in sets {
            let mut set_contents: HashMap<String, u64> = HashMap::new();

            let cubes: Vec<String> = set.split(", ").map(|cube| cube.to_string()).collect();

            for cube in &cubes {
                let splitted: Vec<String> = cube.split(" ").map(|part| part.to_string()).collect();

                let amount: u64 = str::parse::<u64>(&splitted[0]).expect("Failed to parse amount");
                let color = &splitted[1];

                let entry = set_contents.get(&color.to_string());

                let new_amount: u64 = {
                    if let Some(entry) = entry {
                        *entry + amount
                    } else {
                        amount
                    }
                };

                let max_amount = max_cubes_per_color.get(color.as_str());

                if let Some(max_amount) = max_amount {
                    if *max_amount < amount {
                        valid = false;
                    }
                }

                set_contents.insert(color.to_string(), new_amount);
            }
        }

        if valid {
            total += id
        }
    }

    println!("Total: {}", total);
}

fn parse_input(filename: &str) -> Vec<String> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines().map(|line| line.to_string()).collect()
}
