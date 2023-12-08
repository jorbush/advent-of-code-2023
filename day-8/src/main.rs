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

    fn insert_left_at_value(&mut self, target_value: &str, new_value: &str) {
        if self.value == target_value {
            // Insert left child at the current node
            self.insert_left(new_value);
        } else {
            // Search for the target_value in left and right subtrees
            if let Some(left) = &mut self.left {
                left.insert_left_at_value(target_value, new_value);
            }
            if let Some(right) = &mut self.right {
                right.insert_left_at_value(target_value, new_value);
            }
        }
    }

    fn insert_right_at_value(&mut self, target_value: &str, new_value: &str) {
        if self.value == target_value {
            // Insert right child at the current node
            self.insert_right(new_value);
        } else {
            // Search for the target_value in left and right subtrees
            if let Some(left) = &mut self.left {
                left.insert_right_at_value(target_value, new_value);
            }
            if let Some(right) = &mut self.right {
                right.insert_right_at_value(target_value, new_value);
            }
        }
    }

    fn print_in_order(&self, depth: usize) {
        if let Some(right) = &self.right {
            right.print_in_order(depth + 1);
        }

        for _ in 0..depth {
            print!("    ");
        }

        println!("{}", self.value);

        if let Some(left) = &self.left {
            left.print_in_order(depth + 1);
        }
    }
}

fn build_tree_from_lines(iter: &mut std::slice::Iter<String>, root: &mut Option<Box<TreeNode>>) {
    if let Some(line) = iter.next() {
        let root_values: Vec<&str> = line.split('=').collect();
        let root_value = root_values.get(0).unwrap().trim();
        let childs = root_values
            .get(1)
            .unwrap()
            .replace("(", "")
            .replace(")", "");
        let root_childs: Vec<&str> = childs.split(',').map(|s| s.trim()).collect();

        if root.is_none() {
            *root = Some(Box::new(TreeNode::new(root_value)));
        }

        if let Some(left_child) = root_childs.get(0) {
            if let Some(ref mut root_node) = root {
                root_node.insert_left_at_value(root_value, left_child);
            }
        }

        if let Some(right_child) = root_childs.get(1) {
            if let Some(ref mut root_node) = root {
                root_node.insert_right_at_value(root_value, right_child);
            }
        }
        // println!("{:?}", root);
        build_tree_from_lines(iter, root);
    }
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

    let mut iter = nodes.iter();
    let mut root: Option<Box<TreeNode>> = None;
    build_tree_from_lines(&mut iter, &mut root);
    if let Some(root_node) = root {
        root_node.print_in_order(0);
        // println!("{:?}", root_node);
    }
    Ok(())
}

fn main() {
    let _ = part_one();
}
