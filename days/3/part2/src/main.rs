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
    let mut gear_positions: Vec<Position> = Vec::new();
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
                if character == '*' {
                    gear_positions.push(Position {
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

    for gear_position in &gear_positions {
        let mut adjacent_numbers: Vec<&PartNumber> = Vec::new();

        for dx in -1i32..=1 {
            for dy in -1i32..=1 {
                let x = gear_position.x as i32 + dx;
                let y = gear_position.y as i32 + dy;

                let entry = part_numbers.iter().find(|part_number| {
                    part_number.start_x as i32 <= x
                        && part_number.start_x + part_number.value.len() as u32 > x as u32
                        && part_number.start_y == y as u32
                });

                if let Some(part_number) = entry {
                    let existing_entry = adjacent_numbers.iter().find(|num| {
                        num.start_x == part_number.start_x && num.start_y == part_number.start_y
                    });

                    if let None = existing_entry {
                        adjacent_numbers.push(part_number);
                    }
                }
            }
        }

        if adjacent_numbers.len() == 2 {
            total += adjacent_numbers[0].value.parse::<u32>().unwrap()
                * adjacent_numbers[1].value.parse::<u32>().unwrap()
        }
    }

    println!("Total: {total}");
}

fn parse_input(filename: &str) -> Vec<String> {
    let data = fs::read_to_string(filename).expect("Failed to read file");

    data.lines().map(|line| line.to_string()).collect()
}
