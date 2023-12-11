use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn manhattan_distance(start: Point, end: Point) -> usize {
    ((start.x as isize - end.x as isize).abs() + (start.y as isize - end.y as isize).abs()) as usize
}

fn calculate_total_distance(matrix_ids: &Vec<Vec<String>>) -> usize {
    let rows = matrix_ids.len();
    let cols = matrix_ids[0].len();

    let mut total_distance: usize = 0;

    for i in 1..=rows {
        for j in 1..=cols {
            if matrix_ids[i - 1][j - 1] != "." {
                let start = Point { x: i - 1, y: j - 1 };
                for x in 1..=rows {
                    for y in 1..=cols {
                        if matrix_ids[x - 1][y - 1] != "." && (i != x || j != y) {
                            let end = Point { x: x - 1, y: y - 1 };
                            let distance = manhattan_distance(start, end);
                            total_distance += distance;
                        }
                    }
                }
            }
        }
    }

    total_distance
}

fn convert_matrix_ids(
    matrix: &mut Vec<Vec<char>>,
) -> Vec<Vec<String>> {
    let mut count = 1;
    let mut matrix_ids = Vec::new();
    for row in &matrix.clone() {
        let mut ids_row: Vec<String> = Vec::new();
        for cell in row {
            if *cell == '#' {
                let id = count.to_string();
                ids_row.push(id);
                count += 1;
            } else {
                ids_row.push(".".to_string());
            }
        }
        println!("{:?}", ids_row);
        matrix_ids.push(ids_row);
    }
    matrix_ids
}


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
    println!("{}", "With ids: ");
    let mut matrix_ids = convert_matrix_ids(&mut matrix);

    let total_distance = calculate_total_distance(&matrix_ids);
    println!("Total distance between all pairs: {}", total_distance);
    Ok(())
}

fn main() {
    let _ = part_one();
}
