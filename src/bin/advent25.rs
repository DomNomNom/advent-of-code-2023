use itertools::Itertools;
use std::{collections::{HashMap, VecDeque}, fs::read};

fn bfs<'a>(start: &str, to_neighbors: &HashMap<&'a str, Vec<&'a str>>) -> Option<Vec<&'a str>> {
    let to_prev: HashMap<&'a str, &str> = HashMap::new();
    let q = VecDeque::from([start]);
    while let Some(node) = q.pop_front() {
        for neigh in to_neighbors
    }
    None
}

fn main() {
    let filename = "25test.txt";
    let input = read(format!("inputs/{filename}")).unwrap();
    let input = String::from_utf8(input).unwrap();
    let edge_list = input
        .lines()
        .flat_map(|line| {
            let (a, bs) = line.split_once(": ").unwrap();
            bs.split(' ').map(|b| (a, b)).collect_vec()
        })
        .collect_vec();
    let mut to_neighbors: HashMap<&str, Vec<&str>> = HashMap::new();
    for (a, b) in edge_list {
        (*to_neighbors.entry(a).or_default()).push(b);
        (*to_neighbors.entry(b).or_default()).push(a);
        println!("{a} {b}");
    }

    // dbg!(to_neighbors);
    // input.lines
}
