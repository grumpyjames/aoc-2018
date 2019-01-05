extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

struct Node
{
    children: Vec<Node>,
    metadata: Vec<usize>
}

fn main() {
    let file = File::open("src/day8input.txt").unwrap();
    let mut inputs : VecDeque<usize> = VecDeque::new();
    BufReader::new(file)
            .lines()
            .for_each(|line| {
        line.unwrap().split(" ").for_each(|p| inputs.push_back(p.parse().unwrap()));
    });

    let node = parse_node(&mut inputs);

    println!("{}", sum_metadata(&node));
    println!("{}", sum_metadata_two(&node));
}

fn sum_metadata(node: &Node) -> usize {
    let mut result = 0;
    node.metadata.iter().for_each(|i| result += *i);
    node.children.iter().map(sum_metadata).for_each(|i| result += i);
    result
}

fn sum_metadata_two(node: &Node) -> usize {
    let mut result = 0;
    if node.children.is_empty() {
        node.metadata.iter().for_each(|i| result += *i);
    } else {
        node.metadata
            .iter()
            .map(|idx| {
                if *idx <= node.children.len() {
                    sum_metadata_two(&node.children[(*idx) - 1])
                }
                else {
                    0
                }
            })
            .for_each(|i| result += i);
    }
    result
}

fn parse_node(inputs: &mut VecDeque<usize>) -> Node {
    let first = inputs.pop_front().unwrap();
    let second = inputs.pop_front().unwrap();
    let mut children = Vec::new();
    for _i in 0..first {
        children.push(parse_node(inputs))
    }
    let mut metadata = Vec::new();
    for _i in 0..second {
        metadata.push(inputs.pop_front().unwrap());
    }
    Node { children: children, metadata: metadata }
}