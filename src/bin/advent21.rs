use itertools::Itertools;
use std::{
    collections::{vec_deque, HashSet, VecDeque},
    fs::read,
};

fn print_garden(
    rocks: &HashSet<(usize, usize)>,
    possibilities: &HashSet<(usize, usize)>,
    wd: usize,
    ht: usize,
) {
    for row in 0..ht {
        for col in 0..wd {
            let coo = (row, col);
            print!(
                "{}",
                if rocks.contains(&coo) {
                    "#"
                } else if possibilities.contains(&coo) {
                    "O"
                } else {
                    "."
                }
            );
        }
        println!();
    }
    println!();
}

fn main() {
    let input = read("inputs/21.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    let input = input.lines().collect_vec();

    let ht = input.len();
    let wd = input[0].len();

    let mut rocks: HashSet<(usize, usize)> = HashSet::new();
    let mut possibilities: HashSet<(usize, usize)> = HashSet::new();
    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                '#' => {
                    rocks.insert((row, col));
                }
                'S' => {
                    possibilities.insert((row, col));
                }
                _ => unimplemented!(),
            }
        }
    }
    let start = possibilities.clone();

    // part1
    for i in 0..64 {
        possibilities = possibilities
            .iter()
            .flat_map(|&(row, col)| {
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
                if row < wd - 1 {
                    out.push((row, col + 1));
                }
                out
            })
            .filter(|coo| !rocks.contains(coo))
            .collect();
    }
    print_garden(&rocks, &possibilities, wd, ht);
    let answer1 = possibilities.len();
    dbg!(answer1);

    // part2
    // We use the insight that after a certain amount of steps,
    // anything within a certain amount of steps from the start tends to repeat.
    // so then grow a region of space that we don't need to simulate anymore.
    let &(mut s_row, mut s_col) = start.iter().next().unwrap();
    s_row += ((26501365 / ht) + 2) * ht;
    s_col += ((26501365 / wd) + 2) * wd;
    let mut possibilities: HashSet<(usize, usize)> = HashSet::from([(s_row, s_col)]);
    let mut count_histories: VecDeque<[usize; 4]> = VecDeque::new();
    let mut unsimulated_count: [usize; 2] = [0, 0];
    let mut simulated_radius = 0usize;
    count_histories.push_back([0, 0, 0, usize::MAX]); // radius 0
    count_histories.push_back([0, 0, 0, usize::MAX]); // radius 1
    count_histories.push_back([0, 0, 0, usize::MAX]); // radius 2
    count_histories.push_back([0, 0, 0, usize::MAX]); // radius 3

    let radius = |row: &usize, col: &usize| row.abs_diff(s_row) + col.abs_diff(s_col);
    let target_iterations = 50usize;
    for i in 0..target_iterations {
        // update next_count_histories
        let i_mod4 = i % 4;
        for hist4 in count_histories.iter_mut() {
            hist4[i_mod4] = 0;
        }
        for (row, col) in possibilities.iter() {
            let hist_index = radius(row, col) - simulated_radius;
            if hist_index >= count_histories.len() {
                continue;
            }
            count_histories[hist_index][i_mod4] += 1;
        }

        // increase the unsimulated_radius if possible
        while count_histories[0][1] == count_histories[0][3]  // micro optimization: do the check that's more likely to fail first.
            && count_histories[0][0] == count_histories[0][2]
        {
            unsimulated_count[0] += count_histories[0][0];
            unsimulated_count[1] += count_histories[0][1];
            count_histories.pop_front();
            count_histories.push_back([0, 0, 0, usize::MAX]);
            simulated_radius += 1;
            if simulated_radius % 100 == 0 {
                println!("grew at i={i} to r={simulated_radius}");
            }
        }

        // grow the possibilities
        possibilities = possibilities
            .iter()
            .flat_map(|&(row, col)| {
                [
                    (row - 1, col),
                    (row, col - 1),
                    (row + 1, col),
                    (row, col + 1),
                ]
            })
            .filter(|(row, col)| {
                radius(row, col) >= simulated_radius && !rocks.contains(&(row % ht, col % wd))
            })
            .collect();
    }

    dbg!(unsimulated_count[target_iterations % 2]);
    dbg!(possibilities.len());
    let answer2 = unsimulated_count[target_iterations % 2] + possibilities.len();
    dbg!(answer2);
    // let foo = possibilities
    //     .iter()
    //     .map(|(row, col)| radius(row, col))
    //     .collect_vec();
    // dbg!(foo);
    // re-do part1 but with part2 rules to validate.
}
