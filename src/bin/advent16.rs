use itertools::Itertools;
use std::{cmp::max, collections::VecDeque, fs::read, usize};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn bitmask(self) -> u8 {
        match self {
            Dir::N => 0b0001,
            Dir::E => 0b0010,
            Dir::S => 0b0100,
            Dir::W => 0b1000,
        }
    }
}
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Content {
    Empty,
    Slash,
    Backslash,
    SplitVert,
    SplitHori,
}

struct Cell {
    content: Content,
    explored: u8, // bitmask over directions
}

fn energized(
    grid: &Vec<Vec<Content>>,
    start_vel: Dir,
    start_row: usize,
    start_col: usize,
) -> usize {
    let ht = grid.len();
    let wd = grid[0].len();
    let mut explored = grid
        .iter()
        .map(|row| row.iter().map(|_| 0).collect_vec())
        .collect_vec();
    let mut q = VecDeque::from(vec![(start_vel, start_row, start_col)]);
    // note: vel = velocity.
    while let Some((vel, row, col)) = q.pop_front() {
        let content = grid[row][col];

        if explored[row][col] & vel.bitmask() != 0 {
            continue; // already explored
        }
        explored[row][col] |= vel.bitmask();

        let out_vels: Vec<Dir> = match content {
            Content::Empty => vec![vel],
            Content::Slash => match vel {
                Dir::N => vec![Dir::E],
                Dir::E => vec![Dir::N],
                Dir::S => vec![Dir::W],
                Dir::W => vec![Dir::S],
            },
            Content::Backslash => match vel {
                Dir::N => vec![Dir::W],
                Dir::E => vec![Dir::S],
                Dir::S => vec![Dir::E],
                Dir::W => vec![Dir::N],
            },
            Content::SplitVert => match vel {
                Dir::N | Dir::S => vec![vel],
                Dir::W | Dir::E => vec![Dir::N, Dir::S],
            },
            Content::SplitHori => match vel {
                Dir::N | Dir::S => vec![Dir::W, Dir::E],
                Dir::W | Dir::E => vec![vel],
            },
        };

        for v in out_vels {
            if let Some((row2, col2)) = step(&v, row, col, ht, wd) {
                q.push_back((v, row2, col2));
            }
        }
    }

    // for row in explored.iter() {
    //     for &cell in row {
    //         print!("{}", if cell != 0 { "#" } else { " " });
    //     }
    //     println!();
    // }
    explored
        .iter()
        .map(|row| row.iter().filter(|&&cell| cell != 0).count())
        .sum::<usize>()
}

fn main() {
    let input = read("inputs/16.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    let input = input.lines().collect_vec();

    let mut grid = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Content::Empty,
                    '/' => Content::Slash,
                    '\\' => Content::Backslash,
                    '|' => Content::SplitVert,
                    '-' => Content::SplitHori,
                    _ => unimplemented!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let answer1 = energized(&grid, Dir::E, 0, 0);
    dbg!(answer1);

    let ht = grid.len();
    let wd = grid[0].len();
    let foo = [
        (0..ht).map(|z| energized(&grid, Dir::E, 0, z)).max(),
        (0..ht).map(|z| energized(&grid, Dir::W, wd - 1, z)).max(),
        (0..wd).map(|z| energized(&grid, Dir::S, 0, z)).max(),
        (0..wd).map(|z| energized(&grid, Dir::N, z, wd - 1)).max(),
    ];
    let answer2 = foo.iter().map(|z| z.unwrap()).max().unwrap();
    dbg!(answer2);
}
