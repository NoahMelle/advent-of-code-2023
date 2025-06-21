use std::fs;

fn main() {
    let input = parse_input("./src/input.txt");

    for line in input {
        println!("{}", line);
    }
}

fn parse_input(filename: &str) -> Vec<String> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines().map(|line| line.to_string()).collect()
}
