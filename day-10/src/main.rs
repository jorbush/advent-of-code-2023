use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

fn find_start_point(matrix: &Vec<Vec<char>>) -> Option<Point> {
    for (i, row) in matrix.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                return Some(Point { x: i, y: j });
            }
        }
    }
    None
}

fn char_to_distance(ch: char, start_point: Point, matrix: &[Vec<char>]) -> u32 {
    match ch {
        '.' => 0,
        'S' => 0,
        _ => {
            let target_point = find_target_point(ch, matrix);
            manhattan_distance(start_point, target_point.unwrap())
        }
    }
}

fn find_target_point(ch: char, matrix: &[Vec<char>]) -> Result<Point, &'static str> {
    for (i, row) in matrix.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == ch) {
            return Ok(Point { x: i, y: j });
        }
    }
    Err("Objetivo no encontrado en la matriz")
}

fn manhattan_distance(p1: Point, p2: Point) -> u32 {
    ((p1.x as isize - p2.x as isize).abs() + (p1.y as isize - p2.y as isize).abs()) as u32
}

fn find_farthest_point(matrix: &[Vec<u32>], start: Point) -> Point {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut farthest_point = start;
    let mut max_distance = 0;

    while let Some((current_point, distance)) = queue.pop_front() {
        if distance > max_distance {
            max_distance = distance;
            farthest_point = current_point;
        }

        for &(dx, dy) in &directions {
            let new_x = current_point.x as isize + dx;
            let new_y = current_point.y as isize + dy;

            if new_x >= 0 && new_x < rows as isize && new_y >= 0 && new_y < cols as isize {
                let new_point = Point {
                    x: new_x as usize,
                    y: new_y as usize,
                };

                if !visited.contains(&new_point) {
                    visited.insert(new_point);
                    queue.push_back((new_point, distance + matrix[new_point.x][new_point.y]));
                }
            }
        }
    }

    farthest_point
}

fn build_matrix_from_lines(iter: &mut std::slice::Iter<String>, matrix: &mut Vec<Vec<char>>) {
    if let Some(line) = iter.next() {
        let line_values: Vec<char> = line.chars().collect();
        println!("{:?}", line_values);
        matrix.push(line_values);
        build_matrix_from_lines(iter, matrix);
    }
}

fn convert_matrix(matrix: &mut Vec<Vec<char>>, start_point: Point) -> Vec<Vec<u32>> {
    let mut matrix_distances = Vec::new();
    for row in &matrix.clone() {
        let mut distance_row = Vec::new();
        for cell in row {
            let distance = char_to_distance(*cell, start_point, matrix);
            distance_row.push(distance);
        }
        println!("{:?}", distance_row);
        matrix_distances.push(distance_row);
    }
    matrix_distances
}

fn part_one() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    let mut iter = lines.iter();
    let mut matrix: Vec<Vec<char>> = vec![];
    build_matrix_from_lines(&mut iter, &mut matrix);
    if let Some(start_point) = find_start_point(&matrix) {
        println!("{:?}", start_point);
        let matrix_distances = convert_matrix(&mut matrix, start_point);
        // println!("{:?}", matrix_distances);
        //let farthest_point = find_farthest_point(&matrix, start_point);
        // println!("{:?}", farthest_point);
    }

    Ok(())
}

fn main() {
    let _ = part_one();
}
