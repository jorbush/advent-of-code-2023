use std::collections::HashMap;
use std::fs::File;
use std::io::{ self, BufRead };

fn part_one() -> io::Result<()> {

    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut result: u16 = 0;
    let mut seed_maps: Vec<HashMap<u32, u32>> = Vec::new();
    let mut current_map: HashMap<u32, u32> = HashMap::new();
    let mut initial_seeds: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains(":") {
            if initial_seeds.len() > 0 {
                seed_maps.push(current_map.clone());
                println!("New map");
                current_map = HashMap::new();
            }
        } else {
            if initial_seeds.len() == 0 {
                initial_seeds = line.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
            } else {
                let content: Vec<&str> = line.split_whitespace().collect();
                println!("Content: {:?}", content);
                let dest_start: u32 = content.get(0).unwrap().parse().unwrap();
                let source_start: u32 = content.get(1).unwrap().parse().unwrap();
                let range_length: u32 = content.get(2).unwrap().parse().unwrap();

                for i in 0..range_length {
                    let dest_number = dest_start + i;
                    let source_number = source_start + i;
                    current_map.insert(source_number, dest_number);
                }
            }
        }
    }
    println!("Maps: {:?}", seed_maps);
    println!("Result: {}", result);
    Ok(())
}

fn main() {
    let _ = part_one();
}
