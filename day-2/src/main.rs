use std::fs::File;
use std::io::{ self, BufRead };
use regex::Regex;
use std::collections::HashMap;

fn part_one() -> io::Result<()> {
    let mut max_cubes: HashMap<String, u8> = HashMap::new();
    max_cubes.insert("red".to_string(), 12);
    max_cubes.insert("green".to_string(), 13);
    max_cubes.insert("blue".to_string(), 14);

    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let re = Regex::new(r"^Game (\d+)").expect("Error compiling Regex");

    let mut result: u16 = 0;

    for line in reader.lines() {
        let line = line?;
        let mut bad_line = false;
        let content: Vec<&str> = line.split(':').collect();
        // println!("content {:?}", content.get(0));
        if let Some(captures) = re.captures(content.get(0).unwrap()) {
            if let Some(game_number) = captures.get(1) {
                // println!("Game {}", game_number.as_str());
                // for each set
                let sets: Vec<&str> = content.get(1).unwrap().split(';').collect();

                for set in sets {
                    // println!("{:?}", set);
                    for cubes in set.split(",") {
                        let vec_cubes: Vec<&str> = cubes.trim().split_whitespace().collect();
                        // println!("{:?}", vec_cubes.get(0).unwrap());
                        let n = vec_cubes.get(0).unwrap();
                        let key = vec_cubes.get(1).unwrap().to_string();
                        if let Ok(current_number) = n.parse::<u8>() {
                            if let Some(max_value) = max_cubes.get(&key) {
                                // println!("{}", current_number);
                                if current_number > *max_value {
                                    bad_line = true;
                                    break;
                                }
                            }
                        }
                    }
                }
                if !bad_line {
                    if let Ok(game_id) = game_number.as_str().parse::<u16>() {
                        result += game_id;
                    }
                }
            }
        }
    }
    println!("Result: {}", result);
    Ok(())
}

fn main() -> io::Result<()> {
    part_one()
}