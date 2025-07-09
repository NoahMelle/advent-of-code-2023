use std::fs;

#[derive(Debug)]
struct PartNumber {
    start_x: u32,
    start_y: u32,
    value: String,
}

#[derive(Debug)]
struct Position {
    x: u32,
    y: u32,
}

fn main() {
    let input = parse_input("./src/input.txt");
    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let mut symbol_positions: Vec<Position> = Vec::new();
    let mut total = 0;

    let mut current_part_number: Option<&mut PartNumber> = None;

    for (line_index, line) in input.iter().enumerate() {
        for (character_index, character) in line.chars().enumerate() {
            if character.is_numeric() {
                if let Some(part_number) = current_part_number.as_mut() {
                    part_number.value.push_str(&character.to_string());
                } else {
                    part_numbers.push(PartNumber {
                        start_x: character_index as u32,
                        start_y: line_index as u32,
                        value: character.to_string(),
                    });

                    current_part_number = part_numbers.last_mut();
                }
            } else {
                if character != '.' {
                    symbol_positions.push(Position {
                        x: character_index as u32,
                        y: line_index as u32,
                    });
                }

                if let Some(_) = current_part_number {
                    current_part_number = None;
                }
            }
        }

        current_part_number = None
    }

    for part_number in &part_numbers {
        let symbol_entry = symbol_positions.iter().find(|pos| {
            let mut valid = false;

            for i in 0..part_number.value.len() {
                let pos_to_check = Position {
                    x: part_number.start_x + i as u32,
                    y: part_number.start_y,
                };

                if is_pos_adjacent(&pos, &pos_to_check) {
                    valid = true;
                }
            }

            valid
        });

        if let Some(_) = symbol_entry {
            let number = part_number.value.parse::<u32>().unwrap();

            total += number
        }
    }

    println!("{:?}", total);
}

fn is_pos_adjacent(pos1: &Position, pos2: &Position) -> bool {
    let x_distance = (pos1.x as i32 - pos2.x as i32).abs();
    let y_distance = (pos1.y as i32 - pos2.y as i32).abs();

    x_distance <= 1 && y_distance <= 1
}

fn parse_input(filename: &str) -> Vec<String> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines().map(|line| line.to_string()).collect()
}
