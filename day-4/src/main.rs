use std::collections::HashMap;
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

#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {

    fn new(vector: Vec<T>) -> Self {
        Stack { items: vector }
    }

    fn push(&mut self, element: T) {
        self.items.insert(0, element);
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn pop(&mut self) -> Option<T> {
        Some(self.items.remove(0))
    }
}

fn part_two() -> io::Result<()> {

    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut result: u32 = 0;
    let mut scratchcards_results: HashMap<u16, u16> = HashMap::new();
    let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let mut scratchcards = Stack::new(lines.clone());

    while !scratchcards.is_empty() {
        if let Some(scratchcard) = scratchcards.pop() {
            let mut points: u16 = 0;
            result += 1; // increase total instances
            let content: Vec<&str> = scratchcard.split(':').collect();
            let scratchcard_id: Vec<&str> = content.get(0).unwrap().split_whitespace().collect();
            let id_str = scratchcard_id.get(1).unwrap().to_string();
            if let Ok(id) = id_str.parse::<u16>() {
                println!("Card {}", id);
                let numbers: Vec<&str> = content.get(1).unwrap().split('|').collect();
                let current_numbers: Vec<&str> = numbers.get(0).unwrap().split_whitespace().collect();
                let reward_numbers: Vec<&str> = numbers.get(1).unwrap().split_whitespace().collect();
                if scratchcards_results.contains_key(&id) {
                    points = *scratchcards_results.get(&id).unwrap();
                } else {
                    for num in current_numbers {
                        if reward_numbers.contains(&num) {
                            points += 1;
                        }
                    }
                    scratchcards_results.insert(id, points);
                }
                if points > 0 {
                    for reward_line in (0..=points).rev() {
                        let next_index = id as usize + reward_line as usize;
                        if next_index < lines.len() - 1 {
                            if let Some(new_scratchcard) = lines.get(next_index) {
                                scratchcards.push(new_scratchcard.to_string());
                                println!("{}", new_scratchcard)
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Result: {}", result);
    Ok(())
}

fn main() {
    let _ = part_two();
}
