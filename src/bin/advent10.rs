#![allow(dead_code)]
use itertools::Itertools;
use std::{
    collections::VecDeque,
    fs::{self},
};

fn advent10() {
    let input = fs::read("inputs/10.txt").unwrap();
    let lines: Lines = input
        .split(|c| *c == b'\n')
        .map(|line| line.into())
        .collect();

    type Lines = Vec<Vec<u8>>;
    type Coo = (i16, i16); // row, column coordinate

    fn lookup<T: Copy>(lines: &Vec<Vec<T>>, c: &Coo) -> T {
        lines[c.0 as usize][c.1 as usize]
    }
    fn add(a: &Coo, b: &Coo) -> Coo {
        (a.0 + b.0, a.1 + b.1)
    }
    fn is_valid(lines: &Lines, &(row, col): &Coo) -> bool {
        row >= 0
            && col >= 0
            && (row as usize) < lines.len()
            && (col as usize) < lines[row as usize].len()
    }
    fn get_neighbors(lines: &Lines, c: &Coo) -> Vec<Coo> {
        let is_valid2 = |c: &Coo| -> bool { is_valid(lines, c) };
        let offsets: Vec<Coo> = match lookup(lines, c) {
            b'|' => vec![(-1, 0), (1, 0)],
            b'-' => vec![(0, -1), (0, 1)],
            b'L' => vec![(-1, 0), (0, 1)],
            b'J' => vec![(-1, 0), (0, -1)],
            b'7' => vec![(1, 0), (0, -1)],
            b'F' => vec![(1, 0), (0, 1)],
            b'.' => vec![],
            b'\r' => vec![],
            b'S' => {
                return [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .iter()
                    .map(|x| add(x, c))
                    .filter(is_valid2)
                    .filter(|x| get_neighbors(lines, x).iter().any(|z| z == c))
                    .collect();
            }

            _ => unimplemented!(),
        };
        offsets
            .iter()
            .map(|x| add(x, c))
            .filter(is_valid2)
            .collect()
    }

    fn find(needle: u8, haystack: &Lines) -> Option<Coo> {
        for (row, line) in haystack.iter().enumerate() {
            for (col, &x) in line.iter().enumerate() {
                if x == needle {
                    return Some((row as i16, col as i16));
                }
            }
        }
        None
    }

    // Do a floodfill!
    let mut distances: Vec<Vec<i16>> = lines
        .iter()
        .map(|line| line.iter().map(|_| -1).collect())
        .collect();
    let start = find(b'S', &lines).unwrap();
    let mut q = VecDeque::from([(start, 0)]);
    let mut part1_answer = 0;
    while let Some((c, dist)) = q.pop_front() {
        if lookup(&distances, &c) >= 0 {
            continue;
        }
        distances[c.0 as usize][c.1 as usize] = dist;
        part1_answer = dist;
        for n in get_neighbors(&lines, &c) {
            q.push_back((n, dist + 1));
        }
    }
    dbg!(part1_answer);

    // The start being special makes things complicated. normalize it!
    let lines: Lines = lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.iter()
                .enumerate()
                .map(|(col, x)| match *x {
                    b'S' => {
                        let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                            .iter()
                            .filter(|offset| {
                                let coo = &(row as i16, col as i16);
                                let coo_neigh = &add(coo, offset);
                                is_valid(&lines, coo_neigh)
                                    && get_neighbors(&lines, coo_neigh).iter().any(|c| c == coo)
                            })
                            .collect_vec();
                        match offsets[..2] {
                            // pattern matching FTW
                            [(-1, 0), (1, 0)] => b'|',
                            [(0, -1), (0, 1)] => b'-',
                            [(-1, 0), (0, 1)] => b'L',
                            [(-1, 0), (0, -1)] => b'J',
                            [(1, 0), (0, -1)] => b'7',
                            [(1, 0), (0, 1)] => b'F',
                            _ => unreachable!(),
                        }
                    }
                    x => x,
                })
                .collect()
        })
        .collect();

    let mut insides: Vec<Vec<u8>> = lines.clone();

    let mut part2_answer = 0;
    // Scan each row independently, keeping track of when we cross inside and outside.
    for (row, line) in lines.iter().enumerate() {
        let mut inside = 0; // mod4 -> {0 => outside, 1 => on line and above is inside, 2 => inside, 3 => on line and below is inside}
        for (col, &_x) in line.iter().enumerate() {
            let c = &(row as i16, col as i16);
            if lookup(&distances, c) < 0 {
                if inside == 2 {
                    part2_answer += 1;
                }
            } else {
                inside = match lookup(&lines, c) {
                    b'|' => inside + 2,
                    b'-' => inside,
                    b'L' => inside + 1,
                    b'J' => inside + 3,
                    b'7' => inside + 1,
                    b'F' => inside + 3,
                    b'.' => unreachable!(),
                    b'\r' => inside,
                    b'S' => unreachable!(),
                    _ => unreachable!(),
                } % 4
            }
            insides[row][col] = inside;
        }
    }

    // visualize our data.
    for (row, line) in lines.iter().enumerate() {
        for (col, &_x) in line.iter().enumerate() {
            let c = &(row as i16, col as i16);
            if lookup(&distances, c) < 0 {
                print!("{}", insides[row][col]);
            } else {
                print!(
                    "{}",
                    match lookup(&lines, c) {
                        b'|' => "│",
                        b'-' => "─",
                        b'L' => "└",
                        b'J' => "┘",
                        b'7' => "┐",
                        b'F' => "┌",
                        b'.' => " ",
                        b'\r' => " ",
                        b'S' => "S",
                        _ => unreachable!(),
                    }
                );
            }
        }
        println!();
    }

    dbg!(part2_answer);
}

fn main() {
    advent10();
}
