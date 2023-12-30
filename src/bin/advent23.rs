use itertools::Itertools;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet, VecDeque},
    fs::{self, read},
    usize,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}
use Dir::*;

type Coo = (usize, usize);

// fn adjacents((row, col): Coo) -> Vec<Coo> {
//     vec![
//         (row + 1, col),
//         (row, col + 1),
//         (row - 1, col),
//         (row, col - 1),
//     ]
// }

fn main() {
    let filename = "23.txt";
    let input = read(format!("inputs/{filename}")).unwrap();
    let input = String::from_utf8(input).unwrap();
    let input = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let get = |(row, col): Coo| -> char { input[row][col] };
    let ht = input.len();
    let wd = input[0].len();
    let adjacents = |(row, col): Coo| -> Vec<Coo> {
        let mut out = vec![];
        if row > 0 {
            out.push((row - 1, col));
        }
        if col > 0 {
            out.push((row, col - 1));
        }
        if row < ht - 1 {
            out.push((row + 1, col));
        }
        if col < wd - 1 {
            out.push((row, col + 1));
        }
        out
    };

    // check assumption that trails of dots lead between slopes.
    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            match c {
                '>' | '<' | '^' | 'v' => {
                    for coo_next_to_slope in adjacents((row, col)) {
                        if get(coo_next_to_slope) == '.' {
                            // println!(
                            //     "{:?}",
                            // );
                            assert!(
                                adjacents(coo_next_to_slope)
                                    .into_iter()
                                    .filter(|coo| get(*coo) == '.')
                                    .count()
                                    <= 1
                            );
                        }
                    }
                }
                '.' => {
                    assert!(
                        adjacents((row, col))
                            .into_iter()
                            .filter(|coo| get(*coo) == '.')
                            .count()
                            <= 2
                    );
                }
                _ => {}
            }
        }
    }
    // dbg!("yay");

    // group dots into connected regions with a unique ID
    let mut to_id: HashMap<Coo, Coo> = HashMap::new();
    let not_yet_merged = (usize::MAX, usize::MAX);
    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            match c {
                '.' => {
                    to_id.insert((row, col), not_yet_merged);
                }
                _ => {}
            }
        }
    }
    let keys = to_id.keys().map(|k| *k).collect_vec();
    let mut connection_lengths: HashMap<Coo, usize> = HashMap::new();
    for start in keys {
        if to_id[&start] != not_yet_merged {
            continue;
        }
        let mut q = VecDeque::from([start]);
        let mut count = 0;
        while let Some(coo) = q.pop_front() {
            if let Some(v) = to_id.insert(coo, start) {
                if v != not_yet_merged {
                    assert_eq!(v, start);
                    continue; // don't loop back and forth
                }
            }
            count += 1;
            for neighbor in adjacents(coo) {
                if get(neighbor) == '.' {
                    q.push_back(neighbor);
                }
            }
        }
        connection_lengths.insert(start, count);
    }

    // extract connections between the dot paths
    let mut edges: Vec<(Coo, Coo)> = Vec::new();
    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            match c {
                '>' => edges.push((to_id[&(row, col - 1)], to_id[&(row, col + 1)])),
                '<' => edges.push((to_id[&(row, col + 1)], to_id[&(row, col - 1)])),
                '^' => edges.push((to_id[&(row + 1, col)], to_id[&(row - 1, col)])),
                'v' => edges.push((to_id[&(row - 1, col)], to_id[&(row + 1, col)])),
                _ => (),
            };
        }
    }

    // simplify the graph so we have lengths on edges
    let special_top = (999999, 0);
    let special_bot = (0, 999999);
    edges.push((special_top, to_id[&(0, 1)]));
    edges.push(((to_id[&(ht - 1, wd - 2)]), (0, 999999)));
    connection_lengths.insert(special_top, 1);
    connection_lengths.insert(special_bot, 1);

    // // print the graph
    // let mut graph_string = "".to_string();
    // let node_name = |id: &Coo| format!("{},{}:{:?}", id.0, id.1, connection_lengths[id]);
    // for (k, v) in edges.iter() {
    //     graph_string += format!("{} {}\r\n", node_name(k), node_name(v)).as_str();
    // }
    // let _ = fs::write(format!("outputs/{filename}"), graph_string);

    let edges = edges
        .iter()
        .filter_map(|(a, b)| {
            let len_a = connection_lengths[a];
            let len_b = connection_lengths[b];
            assert_eq!(min(len_a, len_b), 1);
            let len = max(len_a, len_b);
            if len_a == 1 {
                // we start at 'a'
                let mut possibilities =
                    edges
                        .iter()
                        .filter_map(|(p, q): &(Coo, Coo)| if p == b { Some((p, q)) } else { None });
                let (p, q) = possibilities.next().unwrap();
                // let len2 = connection_lengths[p];
                assert_eq!(p, b);
                assert_eq!(connection_lengths[q], 1);
                assert_eq!(possibilities.next(), None);
                Some((a, q, len + 3)) // 3 == [>, >, .].len()
            } else {
                None // we should already cover this from the other if-branch
            }
        })
        .collect_vec();

    // print the graph
    let mut graph_string = "".to_string();
    let node_name = |id: &Coo| format!("{},{}", id.0, id.1);
    for (k, v, len) in edges.iter() {
        graph_string += format!("{} {} {len}\r\n", node_name(k), node_name(v)).as_str();
    }
    let _ = fs::write(format!("outputs/{filename}"), graph_string);

    let mut out_edges: HashMap<Coo, Vec<(Coo, usize)>> = HashMap::new();
    for (a, b, len) in edges.clone().into_iter() {
        (*out_edges.entry(*a).or_default()).push((*b, len));
    }

    // find the max distance through it
    fn find_max_dist(out_edges: &HashMap<Coo, Vec<(Coo, usize)>>, start: Coo) -> usize {
        match out_edges.get(&start) {
            Some(outs) => outs
                .iter()
                .map(|(out, len)| find_max_dist(out_edges, *out) + len)
                .max()
                .unwrap(),
            None => 0,
        }
    }

    let answer1 = find_max_dist(&out_edges, special_top) - 4; // the -4 is due to the extra nodes we added.
    dbg!(answer1);

    // optimize things by not caring about coordinates and doing things via indexes.
    let mut coos = edges
        .iter()
        .flat_map(|&(a, b, _len)| [*a, *b])
        .collect_vec();
    coos.sort();
    coos.dedup();
    let to_index: HashMap<Coo, u8> = coos
        .iter()
        .enumerate()
        .map(|(i, coo)| (*coo, i as u8))
        .collect();
    let mut out_edges: HashMap<u8, Vec<(u8, u16)>> = HashMap::new();
    for (a, b, len) in edges.clone().into_iter() {
        (*out_edges.entry(to_index[a]).or_default()).push((to_index[b], len as u16));
    }
    fn find_max_dist_redo(out_edges: &HashMap<u8, Vec<(u8, u16)>>, start: u8) -> u16 {
        match out_edges.get(&start) {
            Some(outs) => outs
                .iter()
                .map(|(out, len)| find_max_dist_redo(out_edges, *out) + len)
                .max()
                .unwrap(),
            None => 0,
        }
    }
    let answer1_redo = find_max_dist_redo(&out_edges, to_index[&special_top]) - 4; // the -4 is due to the extra nodes we added.
    dbg!(answer1_redo);

    // for (a, b, len) in edges.into_iter() {
    //     (*out_edges.entry(*b).or_default()).push((*a, len));
    // }
    // fn find_max_dist2(
    //     out_edges: &HashMap<Coo, Vec<(Coo, usize)>>,
    //     start: Coo,
    //     mut visited: HashSet<Coo>,
    // ) -> usize {
    //     visited.insert(start);
    //     if visited.len() < 15 {
    //         println!("{:?}", visited);
    //     }
    //     match out_edges.get(&start) {
    //         Some(outs) => {
    //             // let mut visited = visited.clone();
    //             outs.iter()
    //                 .filter_map(|(out, len)| {
    //                     if visited.contains(out) {
    //                         None
    //                     } else {
    //                         Some(find_max_dist2(out_edges, *out, visited.clone()) + len)
    //                     }
    //                 })
    //                 .max()
    //                 .unwrap_or_default()
    //         }
    //         None => 0,
    //     }
    // }
    // let answer2 = find_max_dist2(&out_edges, special_top, HashSet::new()) - 4; // the -4 is due to the extra nodes we added.
    // dbg!(answer2);
}
