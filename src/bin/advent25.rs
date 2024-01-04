use itertools::Itertools;
use std::{
    collections::{HashMap, VecDeque},
    fs::read,
};

fn bfs<'a>(
    start: &'a str,
    to_neighbors: &HashMap<&'a str, Vec<&'a str>>,
    end: &'a str,
) -> Option<Vec<&'a str>> {
    let mut to_prev: HashMap<&str, &str> = HashMap::from([(start, start)]);
    let mut q = VecDeque::from([start]);
    while let Some(node) = q.pop_front() {
        if node == end {
            let mut out = vec![end];
            let mut prev = to_prev.get(end);
            while prev.is_some() {
                out.push(prev.unwrap());
                let prev2 = to_prev.get(prev.unwrap());
                if prev2 == prev {
                    break;
                }
                prev = prev2;
            }
            out.reverse();
            return Some(out);
        }
        for neigh in to_neighbors.get(node).unwrap() {
            if !to_prev.contains_key(neigh) {
                to_prev.insert(neigh, node);
                q.push_back(neigh);
            }
        }
    }
    None
}

// fn find_3_cut_between(start: &str, end: &str) -> Option

fn main() {
    let filename = "25.txt";
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
        // println!("{a} {b}");
    }

    let mut nodes = to_neighbors.keys().map(|k| *k).collect_vec();
    nodes.sort();
    let start = nodes.pop().unwrap();
    for end in nodes {
        let mut to_neighbors_try = to_neighbors.clone();
        for _ in 0..3 {
            let path = bfs(start, &to_neighbors_try, end);
            if let Some(path) = path {
                for (a, b) in path.into_iter().tuple_windows::<(_, _)>() {
                    to_neighbors_try.get_mut(a).unwrap().retain(|&x| x != b);
                    to_neighbors_try.get_mut(b).unwrap().retain(|&x| x != a);
                }
            } else {
                // continue 'outer;
                println!("needed fewer than 3 cuts");
                break;
            }
        }
        let path = bfs(start, &to_neighbors_try, end);
        if path.is_some() {
            continue; // need to cut more than 3 links
        }
        println!("found 3-cut partition between {start:?} -> {end:?}");
        // for (k, v) in to_neighbors_try.clone() {
        //     for q in v {
        //         if true || k <= q {
        //             println!("{k} {q}");
        //         }
        //     }
        // }

        // find partition size
        let mut to_prev: HashMap<&str, &str> = HashMap::from([(start, start)]);
        let mut q = VecDeque::from([start]);
        while let Some(node) = q.pop_front() {
            for neigh in to_neighbors_try.get(node).unwrap() {
                // println!("{node} -> {neigh} {to_prev:?}");
                if !to_prev.contains_key(neigh) {
                    to_prev.insert(neigh, node);
                    q.push_back(neigh);
                }
            }
        }
        println!(
            "partition sizes: {} {}",
            to_prev.len(),
            to_neighbors.len() - to_prev.len()
        );
        let answer1 = to_prev.len() * (to_neighbors.len() - to_prev.len());
        dbg!(answer1);
        break;
    }

    // let start = "rsh";
    // let end = "ntq";

    // dbg!(to_neighbors);
    // input.lines
}
