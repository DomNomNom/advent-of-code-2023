use itertools::Itertools;
use std::{
    cmp::{max, min},
    collections::{HashSet, VecDeque},
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

fn step(row: i32, col: i32, dir: &Dir, dist: i32) -> (i32, i32) {
    // this looks nicer on one line per direction but I guess I'll let the formatter do its job
    match dir {
        Dir::N => (row - dist as i32, col),
        Dir::E => (row, col + dist as i32),
        Dir::S => (row + dist as i32, col),
        Dir::W => (row, col - dist as i32),
    }
}

fn part1solver(instructions: &Vec<(Dir, i32, u32)>) -> usize {
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut min_row: i32 = 0;
    let mut max_row: i32 = 0;
    let mut min_col: i32 = 0;
    let mut max_col: i32 = 0;
    for &(dir, dist, _) in instructions.iter() {
        (row, col) = step(row, col, &dir, dist);
        max_row = max(max_row, row);
        max_col = max(max_col, col);
        min_row = min(min_row, row);
        min_col = min(min_col, col);
    }
    let ht = max_row - min_row + 1;
    let wd = max_col - min_col + 1;

    let mut grid = (0..ht)
        .map(|_| (0..wd).map(|_| 0u32).collect_vec())
        .collect_vec();

    for &(dir, dist, color) in instructions.iter() {
        for i in 0..dist {
            let (row2, col2) = step(row, col, &dir, i + 1);
            grid[(row2 - min_row) as usize][(col2 - min_col) as usize] = color;
        }
        (row, col) = step(row, col, &dir, dist);
    }

    // floodfill from center
    let mut q = VecDeque::from(vec![((wd / 2) as usize, (ht / 2) as usize)]);
    while let Some((row, col)) = q.pop_back() {
        if grid[row][col] != 0 {
            continue;
        }
        grid[row][col] = 0xff000000;
        q.push_back((row + 1, col));
        q.push_back((row - 1, col));
        q.push_back((row, col + 1));
        q.push_back((row, col - 1));
    }

    // for line in grid.iter() {
    //     for cell in line {
    //         print!(
    //             "{}",
    //             match cell {
    //                 0 => '.',
    //                 0xff000000 => 'a',
    //                 _ => '#',
    //             }
    //         );
    //     }
    //     println!();
    // }
    return grid
        .iter()
        .map(|line| {
            line.iter()
                .map(|&cell| if cell == 0 { 0 } else { 1 })
                .sum::<usize>()
        })
        .sum::<usize>();
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Rect {
    start_row: i32,
    start_col: i32,
    ht: i32,
    wd: i32,
    is_entry_or_exit: bool,
}

fn intersecting(a: &Rect, b: &Rect) -> bool {
    true // style
        && a.start_row < b.start_row + b.ht
        && b.start_row < a.start_row + a.ht
        && a.start_col < b.start_col + b.wd
        && b.start_col < a.start_col + a.wd
}

fn touching(a: &Rect, b: &Rect) -> bool {
    true  // style
        && a.start_row < b.start_row + b.ht + 1
        && b.start_row < a.start_row + a.ht + 1
        && a.start_col < b.start_col + b.wd + 1
        && b.start_col < a.start_col + a.wd + 1
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
enum StartEnd {
    Start,
    End,
}
use StartEnd::*;

fn part2solver(instructions: &Vec<(Dir, i32)>) -> i64 {
    let mut row: i32 = 0;
    let mut col: i32 = 0;
    let mut rects = vec![];
    for ((prev_dir, _), &(dir, dist), (next_dir, _)) in instructions[instructions.len() - 1..]
        .iter()
        .chain(instructions.iter().chain(instructions[0..1].iter()))
        .tuple_windows::<(_, _, _)>()
    {
        let (row2, col2) = step(row, col, &dir, dist);
        // make our life easy later by making vertical lines short and horizontal ones long.
        // this means we can determine from each rectangle whether it is an entry or exit to the overall shape.
        rects.push(match dir {
            N => Rect {
                start_row: row2 + 1,
                start_col: col,
                ht: dist - 1,
                wd: 1,
                is_entry_or_exit: true,
            },
            E => Rect {
                start_row: row,
                start_col: col,
                ht: 1,
                wd: dist + 1,
                is_entry_or_exit: prev_dir == next_dir,
            },
            S => Rect {
                start_row: row + 1,
                start_col: col,
                ht: dist - 1,
                wd: 1,
                is_entry_or_exit: true,
            },
            W => Rect {
                start_row: row,
                start_col: col2,
                ht: 1,
                wd: dist + 1,
                is_entry_or_exit: prev_dir == next_dir,
            },
        });
        (row, col) = (row2, col2);
    }

    for (i, r) in rects.iter().enumerate() {
        let next = &rects[(i + 1) % rects.len()];
        assert!(touching(r, next));
        assert!(!intersecting(r, next));
        // let touchings = rects.iter().filter(|r2| touching(r, r2)).collect_vec();
        // assert!(touchings.len() == 3);
    }

    // now get them sorted primarily by start row
    rects.sort();

    for (i, r) in rects.iter().enumerate() {
        for r2 in rects[i + 1..].iter() {
            assert!(!intersecting(r, r2));
        }
    }

    // some assumptions that'll make our code easier.
    for r in rects.iter() {
        let intersectings = rects.iter().filter(|r2| intersecting(r, r2)).collect_vec();
        assert!(intersectings.len() == 1);
    }

    let get_row = |(startend, r): &(StartEnd, Rect)| match startend {
        Start => r.start_row,
        End => r.start_row + r.ht, // note: this is the first row after the rect.
    };
    let vertical_ranges = &rects
        .into_iter()
        // .flat_map(|r| [(r.start_row, Start, r), (r.start_row + r.ht, End, r)])
        .flat_map(|r| [(Start, r), (End, r)])
        .sorted_by_key(get_row)
        .group_by(get_row);

    let mut active_rects = HashSet::new();
    let mut active_column_count = 0;
    let mut prev_row = i32::MIN;
    let mut answer: i64 = 0;
    for (row, stuff) in vertical_ranges {
        if prev_row > i32::MIN {
            answer += ((row - prev_row) as i64 * active_column_count) as i64;
        }

        // update active_recs
        for (se, rect) in stuff {
            match se {
                Start => assert!(active_rects.insert(rect)),
                End => assert!(active_rects.remove(&rect)),
            };
        }

        // update active_column_count
        active_column_count = 0;
        prev_row = row;
        let mut inside = false;
        let mut left = 0;
        for rect in active_rects.iter().sorted_by_key(|&r| r.start_col) {
            if inside {
                if rect.is_entry_or_exit {
                    active_column_count += ((rect.start_col + rect.wd) - left) as i64;
                    inside = false;
                }
                // no else - avoid double-counting a rect that is not a entry or exit that is already contained.
            } else {
                if rect.is_entry_or_exit {
                    left = rect.start_col;
                    inside = true;
                } else {
                    active_column_count += rect.wd as i64;
                }
            }
        }
        assert!(!inside);
    }
    assert_eq!(active_rects.len(), 0);

    answer
}

fn main() {
    let input = read("inputs/18.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    let input = input.lines().collect_vec();
    let instructions: Vec<(Dir, i32, u32)> = input
        .clone()
        .into_iter()
        .map(|line| {
            let (step, color) = line.split_once(" (#").unwrap();
            let (dir, distance) = step.split_once(" ").unwrap();
            let dir = match dir {
                "U" => N,
                "R" => E,
                "D" => S,
                "L" => W,
                x => unimplemented!("{}", x),
            };
            let distance: i32 = distance.parse().unwrap();
            let color = &color[..6];
            let color: u32 = u32::from_str_radix(color, 16).unwrap();
            (dir, distance, color)
        })
        .collect_vec();
    let answer1 = part1solver(&instructions);
    dbg!(answer1);

    let instructions1 = instructions.iter().map(|&(a, b, _)| (a, b)).collect_vec();
    let answer1_ = part2solver(&instructions1);
    dbg!(answer1_);

    let instructions2 = input
        .into_iter()
        .map(|line| {
            let (_, color) = line.split_once("#").unwrap();
            let dir = match &color[5..6] {
                "0" => E,
                "1" => S,
                "2" => W,
                "3" => N,
                _ => unimplemented!(),
            };
            let dist = i32::from_str_radix(&color[..5], 16).unwrap();
            (dir, dist)
        })
        .collect_vec();
    let answer2 = part2solver(&instructions2);
    dbg!(answer2);
}
