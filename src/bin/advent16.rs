use itertools::Itertools;
use std::{collections::VecDeque, fs::read, usize};

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

fn main() {
    let input = read("inputs/16.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    let input = input.lines().collect_vec();

    let mut grid = input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| Cell {
                    explored: 0,
                    content: match c {
                        '.' => Content::Empty,
                        '/' => Content::Slash,
                        '\\' => Content::Backslash,
                        '|' => Content::SplitVert,
                        '-' => Content::SplitHori,
                        _ => unimplemented!(),
                    },
                })
                .collect_vec()
        })
        .collect_vec();
    let ht = grid.len();
    let wd = grid[0].len();
    let mut q = VecDeque::from(vec![(Dir::E, 0usize, 0usize)]);
    // note: vel = velocity.
    while let Some((vel, row, col)) = q.pop_front() {
        let cell = &mut grid[row][col];

        if cell.explored & vel.bitmask() != 0 {
            continue; // already explored
        }
        cell.explored |= vel.bitmask();

        let out_vels: Vec<Dir> = match cell.content {
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

    for row in grid.iter() {
        for cell in row {
            print!("{}", if cell.explored != 0 { "#" } else { " " });
        }
        println!();
    }
    let answer1 = grid
        .iter()
        .map(|row| row.iter().filter(|cell| cell.explored != 0).count())
        .sum::<usize>();
    dbg!(answer1); // 7608 too low
}
