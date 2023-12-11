use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io;
use std::io::BufRead;

fn build_matrix_from_lines(iter: &mut std::slice::Iter<String>, matrix: &mut Vec<Vec<char>>) {
    if let Some(line) = iter.next() {
        let line_values: Vec<char> = line.chars().collect();
        println!("{:?}", line_values);
        matrix.push(line_values);
        build_matrix_from_lines(iter, matrix);
    }
}

fn duplicate_empty_rows(matrix: &mut Vec<Vec<char>>) {
    let mut new_rows = matrix.clone();

    for (index, row) in matrix.iter().enumerate() {
        if row.iter().all(|&cell| cell == '.') {
            new_rows.insert(index + 1, row.clone());
        }
    }

    *matrix = new_rows;
}

fn duplicate_empty_columns(matrix: &mut Vec<Vec<char>>) {
    let mut new_columns = Vec::new();

    for col in 0..matrix[0].len() {
        if matrix.iter().all(|row| row[col] == '.') {
            new_columns.push(col);
        }
    }

    for &col in new_columns.iter().rev() {
        for row in matrix.iter_mut() {
            row.insert(col + 1, '.');
        }
    }
}

fn part_one() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    let mut iter = lines.iter();
    let mut matrix: Vec<Vec<char>> = vec![];
    build_matrix_from_lines(&mut iter, &mut matrix);
    duplicate_empty_rows(&mut matrix);
    duplicate_empty_columns(&mut matrix);
    println!("{}", "Duplicated empty rows and columns: ");
    for row in matrix.iter() {
        println!("{:?}", row);
    }
    Ok(())
}

fn main() {
    let _ = part_one();
}
