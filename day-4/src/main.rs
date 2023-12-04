use std::fs::File;
use std::io::{ self, BufRead };

fn part_one() -> io::Result<()> {

    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut result: u16 = 0;

    for line in reader.lines() {
        let line = line?;
        let mut points: u16 = 0;
        let content: Vec<&str> = line.split(':').collect();
        let numbers: Vec<&str> = content.get(1).unwrap().split('|').collect();
        let current_numbers: Vec<&str> = numbers.get(0).unwrap().split_whitespace().collect();
        let reward_numbers: Vec<&str> = numbers.get(1).unwrap().split_whitespace().collect();
        for num in current_numbers {
            if reward_numbers.contains(&num) {
                if points > 0 {
                    points *= 2;
                } else {
                    points = 1;
                }
            }
        }
        result += points;
    }
    println!("Result: {}", result);
    Ok(())
}

fn main() {
    let _ = part_one();
}
