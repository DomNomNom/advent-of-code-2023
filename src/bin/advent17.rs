use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

use Dir::*;
fn step(d: &Dir, row: usize, col: usize, ht: usize, wd: usize) -> Option<(usize, usize)> {
    // this looks nicer on one line per direction but I guess I'll let the formatter do its job
    match d {
        Dir::N => {
            if row > 0 {
                Some((row - 1, col))
            } else {
                None
            }
        }
        Dir::E => {
            if col < wd - 1 {
                Some((row, col + 1))
            } else {
                None
            }
        }
        Dir::S => {
            if row < ht - 1 {
                Some((row + 1, col))
            } else {
                None
            }
        }
        Dir::W => {
            if col > 0 {
                Some((row, col - 1))
            } else {
                None
            }
        }
    }
}

fn min_crucible_path(
    cost_grid: &Vec<Vec<u8>>,
    min_straight_steps: u8,
    max_straight_steps: u8,
) -> usize {
    // let mut q = VecDeque::from(vec![(0usize, 0usize, 3u8, N, 0usize)]);
    let mut q = PriorityQueue::new();
    q.push((0usize, 0usize, 0u8, E), 0i32);
    q.push((0usize, 0usize, 0u8, S), 0i32);

    let ht = cost_grid.len();
    let wd = cost_grid[0].len();
    let mut to_prev: HashMap<(usize, usize, u8, Dir), (usize, usize, u8, Dir)> = HashMap::new();
    let mut last_tup = None;
    let mut min_heatloss = 0usize;
    while let Some((tup, priority)) = q.pop() {
        // println!("a {tup:?} {priority}");
        let (row, col, vel_steps_done, vel) = tup;
        if row == ht - 1 && col == wd - 1 && vel_steps_done >= min_straight_steps {
            last_tup = Some(tup);
            min_heatloss = -priority as usize;
            break;
        }

        let mut out_vels = match vel {
            N | S => vec![E, W],
            W | E => vec![N, S],
        };
        if vel_steps_done < min_straight_steps {
            out_vels = vec![];
        }
        if vel_steps_done < max_straight_steps {
            out_vels.push(vel);
        }
        for vel2 in out_vels {
            let pos2 = step(&vel2, row, col, ht, wd);
            if pos2.is_none() {
                continue;
            }
            let (row2, col2) = pos2.unwrap();
            let priority2 = priority - cost_grid[row2][col2] as i32;
            let vel_steps_done_2 = if vel2 == vel { vel_steps_done + 1 } else { 1 };

            let tup2 = (row2, col2, vel_steps_done_2, vel2);
            if to_prev.get(&tup2).is_some() {
                continue;
            }
            q.push(tup2, priority2);
            to_prev.insert(tup2, tup);
        }
    }
    assert!(last_tup.is_some());

    // visualize the answer
    let mut gridArt = cost_grid
        .iter()
        .map(|line| line.iter().map(|_| '.').collect_vec())
        .collect_vec();
    while let Some((row, col, vel_steps_done, vel)) = last_tup {
        gridArt[row][col] = '#';
        let foo = last_tup.unwrap();
        let bar = to_prev.get(&foo);
        if row == 0 && col == 0 {
            break;
        }
        last_tup = match bar {
            None => None,
            Some(&tup2) => Some(tup2),
        }
    }
    for line in gridArt {
        for c in line {
            print!("{}", c);
        }
        println!();
    }

    return min_heatloss;
}

fn main() {
    let input = read("inputs/17.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    let input = input.lines().collect_vec();
    let cost_grid = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect_vec()
        })
        .collect_vec();

    let answer_1 = min_crucible_path(&cost_grid, 0, 3);
    dbg!(answer_1);
    let answer_2 = min_crucible_path(&cost_grid, 4, 10);
    dbg!(answer_2);
}
