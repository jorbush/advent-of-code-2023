use std::fs::File;
use std::io::{ self, BufRead };

fn part_one() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut lines_iter = reader.lines();
    let mut result: u32 = 0;



    println!("Result: {}", result);

    Ok(())
}

fn main() {
    let _ = part_one();
}