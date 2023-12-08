use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct TreeNode {
    value: String,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: &str) -> Self {
        TreeNode {
            value: value.to_string(),
            left: None,
            right: None,
        }
    }

    fn insert_left(&mut self, value: &str) {
        if let Some(left) = &mut self.left {
            left.insert_left(value);
        } else {
            self.left = Some(Box::new(TreeNode::new(value)));
        }
    }

    fn insert_right(&mut self, value: &str) {
        if let Some(right) = &mut self.right {
            right.insert_right(value);
        } else {
            self.right = Some(Box::new(TreeNode::new(value)));
        }
    }

    fn print_in_order(&self) {
        if let Some(left) = &self.left {
            left.print_in_order();
        }
        println!("{}", self.value);
        if let Some(right) = &self.right {
            right.print_in_order();
        }
    }

    fn height(&self) -> usize {
        1 + usize::max(
            self.left.as_ref().map_or(0, |node| node.height()),
            self.right.as_ref().map_or(0, |node| node.height()),
        )
    }
}

fn build_tree_from_lines(lines: &[String]) -> Option<Box<TreeNode>> {
    if lines.is_empty() {
        return None;
    }

    let root_line = &lines[0];
    let root_values: Vec<&str> = root_line.split('=').collect();
    let root_value = root_values.get(0).unwrap().trim();
    let childs = root_values
        .get(1)
        .unwrap()
        .replace("(", "")
        .replace(")", "");
    let root_childs: Vec<&str> =
        childs
        .split(',')
        .map(|s| s.trim())
        .collect();

    let left_child = root_childs.get(0).unwrap();
    let right_child = root_childs.get(1).unwrap();
    let mut root = TreeNode::new(root_value);
    root.insert_left(left_child);
    root.insert_right(right_child);

    // continue adding

    Some(Box::new(root))
}

fn part_one() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let file2 = File::open("input.txt")?;
    let reader = io::BufReader::new(file);
    let reader2 = io::BufReader::new(file2);

    let left_right_instructions = reader.lines().next().unwrap()?.trim().to_string();
    println!("Instructions {}", left_right_instructions);
    let nodes: Vec<String> = reader2
        .lines()
        .skip(2)
        .filter_map(|line| line.ok())
        .collect();

    if let Some(root) = build_tree_from_lines(&nodes) {
        root.print_in_order();
    }

    Ok(())
}

fn main() {
    let _ = part_one();
}
