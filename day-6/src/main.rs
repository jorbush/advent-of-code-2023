use std::fs::File;
use std::io::{ self, BufRead };

fn part_one() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut lines_iter = reader.lines();
    let mut result: u16 = 1;

    if let Some(Ok(line1)) = lines_iter.next() {
        let content_time: Vec<&str> = line1.split(':').collect();
        let times: Vec<&str> = content_time.get(1).unwrap().split_whitespace().collect();

        println!("Times: {:?}", times);

        if let Some(Ok(line2)) = lines_iter.next() {
            let content_distance: Vec<&str> = line2.split(':').collect();
            let distances: Vec<&str> = content_distance.get(1).unwrap().split_whitespace().collect();
            println!("Distances: {:?}", distances);
            let mut ways: u16 = 0;
            for (index, &time_str) in times.iter().enumerate() {
                if let Ok(time) = time_str.parse::<u16>() {
                    if let Some(record_distance_str) = distances.get(index) {
                        if let Ok(record_distance) = record_distance_str.parse::<u16>() {
                            for holding_time in 0..=time {
                                println!("Holding time {}", holding_time);
                                let new_record = holding_time * (time - holding_time);
                                println!("New record {}", new_record);
                                if new_record > record_distance {
                                    ways += 1;
                                    println!("Ways: {}", ways);
                                }
                            }
                        }
                    }
                }
                result *= ways;
                ways = 0;
            }
        }
    }

    println!("Result: {}", result);

    Ok(())
}

fn main() {
    let _ = part_one();
}