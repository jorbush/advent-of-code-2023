use std::fs::File;
use std::io::{ self, BufRead };

fn part_one() -> io::Result<()> {

    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut result: u32 = 0;
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let mut acc = String::new();
    let mut adjacent: bool = false;

    for (line_ind, line) in lines.iter().enumerate() {
        for (char_position, character) in line.chars().enumerate() {

            if character == '.' {
                acc.clear();
                adjacent = false;
            }
            if character.is_numeric() {
                acc = acc + &character.to_string();
                let mut finish_acc = false;
                // Check previous line
                if line_ind > 0 {
                    if let Some(previous_line) = lines.get(line_ind - 1) {
                        if let Some(next_char) = previous_line.chars().nth(char_position) {
                            if next_char != '.' && !next_char.is_numeric() {
                                adjacent = true;
                            }
                        }
                        if char_position > 0 {
                            if let Some(next_char) = previous_line.chars().nth(char_position - 1) {
                                if next_char != '.' && !next_char.is_numeric() {
                                    adjacent = true;
                                }
                            }
                        }
                        if let Some(next_char) = previous_line.chars().nth(char_position + 1) {
                            if next_char != '.' && !next_char.is_numeric() {
                                adjacent = true;
                            }
                        }
                    }
                }

                // Check current line
                if let Some(next_char) = line.chars().nth(char_position + 1) {
                    if next_char != '.' && !next_char.is_numeric() {
                        adjacent = true;
                    }
                    if !next_char.is_numeric() {
                        finish_acc = true;
                    }
                }
                if char_position > 0 {
                    if let Some(next_char) = line.chars().nth(char_position - 1) {
                        if next_char != '.' && !next_char.is_numeric() {
                            adjacent = true;
                        }
                    }
                }

                // Check next line
                if let Some(next_line) = lines.get(line_ind + 1) {
                    if let Some(next_char) = next_line.chars().nth(char_position) {
                        if next_char != '.' && !next_char.is_numeric() {
                            adjacent = true;
                        }
                    }
                    if char_position > 0 {
                        if let Some(next_char) = next_line.chars().nth(char_position - 1) {
                            if next_char != '.' && !next_char.is_numeric() {
                                adjacent = true;
                            }
                        }
                    }
                    if let Some(next_char) = next_line.chars().nth(char_position + 1) {
                        if next_char != '.' && !next_char.is_numeric() {
                            adjacent = true;
                        }
                    }
                }

                // When complete sequence, check if is adjacent to sum
                if finish_acc {
                    println!("{} {}", acc, adjacent);
                    if adjacent {
                        if let Ok(acc) = acc.parse::<u32>() {
                            result += acc;
                        }
                        acc.clear();
                    }
                }

            }
        }
        println!("{}", "----------");
    }
    println!("Result: {}", result);
    Ok(())
}

fn main() -> io::Result<()> {
    part_one()
}
