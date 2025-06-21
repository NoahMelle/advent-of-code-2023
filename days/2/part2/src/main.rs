use std::fs;

struct MinContents {
    blue: u64,
    red: u64,
    green: u64,
}

fn main() {
    let input = parse_input("./src/input.txt");

    let mut total: u64 = 0;

    for game in input {
        let parts: Vec<&str> = game.split(": ").collect();
        let (_, line) = (parts[0], parts[1]);

        let sets = line.split("; ");

        let mut min_game_contents = MinContents {
            green: 0,
            blue: 0,
            red: 0,
        };

        for set in sets {
            let cubes: Vec<String> = set.split(", ").map(|cube| cube.to_string()).collect();

            for cube in &cubes {
                let splitted: Vec<String> = cube.split(" ").map(|part| part.to_string()).collect();

                let amount: u64 = str::parse::<u64>(&splitted[0]).expect("Failed to parse amount");
                let color = &splitted[1];

                match color.as_str() {
                    "blue" => {
                        if amount > min_game_contents.blue {
                            min_game_contents.blue = amount;
                        }
                    }
                    "red" => {
                        if amount > min_game_contents.red {
                            min_game_contents.red = amount;
                        }
                    }
                    "green" => {
                        if amount > min_game_contents.green {
                            min_game_contents.green = amount;
                        }
                    }
                    _ => panic!("Unknown color: {}", color),
                }
            }
        }

        total += min_game_contents.blue * min_game_contents.red * min_game_contents.green;
    }

    println!("Total: {}", total);
}

fn parse_input(filename: &str) -> Vec<String> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines().map(|line| line.to_string()).collect()
}
