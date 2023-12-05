use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::{ self, BufRead };
use std::thread;

fn calculate_final_value(seed: u32, seed_maps: &Vec<HashMap<u32, u32>>) -> u32 {
    let mut current_value = seed;
    for map in seed_maps {
        if let Some(&dest_number) = map.get(&current_value) {
            current_value = dest_number;
        }
    }
    current_value
}

fn process_block(
    dest_start: u32,
    source_start: u32,
    range_length: u32,
    current_map: &mut HashMap<u32, u32>,
) {
    for i in 0..range_length {
        let dest_number = dest_start + i;
        let source_number = source_start + i;
        // println!("{:?} {:?}", dest_number, source_number);
        current_map.insert(source_number, dest_number);
    }
}

fn part_one() -> io::Result<()> {

    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut result: u32 = std::u32::MAX;
    let mut seed_maps: Vec<HashMap<u32, u32>> = Vec::new();
    let mut current_map: HashMap<u32, u32> = HashMap::new();
    let mut initial_seeds: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        if line.contains(":") {
            if initial_seeds.len() > 0 {
                seed_maps.push(current_map.clone());
                println!("New map");
                current_map.clear();
            } else {
                let content: Vec<&str> = line.split(':').collect();
                let seeds: Vec<&str> = content.get(1).unwrap().split_whitespace().collect();
                initial_seeds = seeds.iter().map(|s| s.parse::<u32>().unwrap()).collect();
            }
        } else {
            let content: Vec<&str> = line.split_whitespace().collect();
            println!("Content: {:?}", content);
            let dest_start: u32 = content.get(0).unwrap().parse().unwrap();
            let source_start: u32 = content.get(1).unwrap().parse().unwrap();
            let range_length: u32 = content.get(2).unwrap().parse().unwrap();

            let num_threads = 4;
            let chunk_size = range_length / num_threads;
            let handles: Vec<_> = (0..num_threads)
                .map(|i| {
                    let dest_start = dest_start + i * chunk_size;
                    let source_start = source_start + i * chunk_size;
                    let range_length = if i == num_threads - 1 {
                        range_length - i * chunk_size
                    } else {
                        chunk_size
                    };

                    let current_map_clone = current_map.clone();
                    let handle = thread::spawn(move || {
                        let mut local_map = current_map_clone;
                        process_block(dest_start, source_start, range_length, &mut local_map);
                        local_map
                    });

                    handle
                })
                .collect();

            let thread_maps: Vec<_> = handles
                .into_iter()
                .map(|handle| handle.join().unwrap())
                .collect();

            for local_map in thread_maps {
                current_map.extend(local_map);
            }
        }
    }
    println!("Maps: {:?}", seed_maps);
    for seed in &initial_seeds {
        let final_value = calculate_final_value(*seed, &seed_maps);
        println!("Initial Seed: {}, Final Value: {}", seed, final_value);
        result = min(result, final_value);
    }
    println!("Result: {}", result);
    Ok(())
}

fn main() {
    let _ = part_one();
}
