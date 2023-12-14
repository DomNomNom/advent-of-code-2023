#![allow(dead_code)]
use counter::Counter;
use itertools::Itertools;
use regex::Regex;
use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet, VecDeque},
    env,
    fs::{self, read, File},
    io::{BufRead, BufReader, Read},
    iter::zip,
    ops::Range,
    vec,
};

use memoize::memoize;

fn parse(digit: &str) -> u32 {
    match digit {
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        // "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        d => panic!("can't parse digit: {d:?}"),
    }
}

fn advent01() {
    println!("{:?}", env::current_dir());
    let file = File::open("inputs/01.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line: String = String::new();
    let mut acc = 0u32;

    let r1 = Regex::new(r"([0-9])|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)")
        .unwrap();
    let r2 = Regex::new(r"([0-9])|(eno)|(owt)|(eerht)|(ruof)|(evif)|(xis)|(neves)|(thgie)|(enin)")
        .unwrap();
    //(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)
    // "one two three four five six seven eight nine"
    while {
        line.clear();
        buf_reader.read_line(&mut line).unwrap()
    } > 0
    {
        let mut lineout = 0u32;
        // let (m1, _) = r1.captures_iter(&line).next().unwrap().extract();
        let (_, [m1]) = r1.captures_iter(&line).next().unwrap().extract();
        // println!("{m1:?} {m2:?}");
        lineout += 10 * parse(m1);

        let rev = line.chars().rev().collect::<String>();
        let (_, [m2]) = r2.captures_iter(&rev).next().unwrap().extract();
        let ver = m2.chars().rev().collect::<String>();
        lineout += parse(ver.as_str());

        // let numerics: Vec<_> = line.chars().filter(|c| c.is_numeric()).collect();
        // let mut lineout = 10 * numerics[0].to_digit(10).unwrap()
        //     + (*numerics.last().unwrap()).to_digit(10).unwrap();
        println!("{} -> {}", line, lineout);
        acc += lineout;
    }
    println!("{acc}");
}

fn advent02() {
    let file = File::open("inputs/02.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line: String = String::new();
    let mut acc = 0u32;

    while {
        line.clear();
        buf_reader.read_line(&mut line).unwrap()
    } > 0
    {
        let (_game_str, samples) = line.split_once(": ").unwrap();

        // if samples.split("; ").all(|sample| {
        //     sample.trim().split(", ").all(|count_color| {
        //         let (count_str, color) = count_color.split_once(" ").unwrap();
        //         let count: u32 = count_str.parse().unwrap();

        //         if color == "red" {
        //             count <= 12
        //         } else if color == "green" {
        //             count <= 13
        //         } else if color == "blue" {
        //             count <= 14
        //         } else {
        //             panic!("bad color: {color:?}")
        //         }
        //     })
        // }) {
        //     let (_, game_int_str) = game_str.split_once(' ').unwrap();
        //     let game: u32 = game_int_str.parse().unwrap();
        //     acc += game;
        // }

        let count_colors = samples
            .split(';')
            .map(|sample| {
                sample.trim().split(',').map(|count_color| {
                    let (count, color) = count_color.trim().split_once(" ").unwrap();
                    (
                        count.parse::<u32>().unwrap(),
                        ["red", "green", "blue"]
                            .iter()
                            .position(|&c| c == color)
                            .unwrap(),
                    )
                })
            })
            .flatten();
        let mut maxs = [0, 0, 0];
        for (count, color) in count_colors.clone() {
            if count > maxs[color] {
                maxs[color] = count
            }
        }
        // let count_colors = count_colors.collect::<Vec<_>>();
        // println!(
        //     "{samples:?} {count_colors:?} {maxs:?} {}",
        //     maxs.iter().product::<u32>()
        // );
        acc += maxs.iter().product::<u32>();
    }

    print!("{acc}");
}

// struct ContextIterator<I, T> {
//     iter: I,
//     prev: Option<T>,
//     now: Option<T>,
//     next: Option<T>,
// }
// impl<I, T> ContextIterator<I, T>
// where
//     I: Iterator<Item = T>,
// {
//     fn new(mut iter: I) -> ContextIterator<I, T> {
//         ContextIterator {
//             iter: iter,
//             prev: None,
//             now: None,
//             next: iter.next(),
//         }
//     }
// }
// impl<'a, I: Iterator, T> Iterator for ContextIterator<I, T>
// where
//     I: Iterator<Item = T>,
// {
//     type Item = (Option<&'a mut T>, &'a mut T, Option<&'a mut T>);

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.next {
//             None => None,
//             Some(next) => {}
//         }
//     }
// }

// fn map_with_context<T>(it: Iterator<T>, f: FnMut<&'a mut T>)

fn advent03() {
    println!("{:?}", env::current_dir());
    let input = fs::read_to_string("inputs/03.txt").unwrap();
    let grid: Vec<Vec<char>> = input
        .split('\n')
        .map(|line| line.trim().chars().collect())
        .collect();
    let mut used: Vec<Vec<bool>> = grid
        .iter()
        .map(|line| line.iter().map(|_| false).collect())
        .collect();
    let mut acc = 0u32;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if used[i][j] || grid[i][j] == '.' || grid[i][j].is_numeric() {
                continue;
            }
            used[i][j] = true;
            // scan around the marker
            for ii in (max(i, 1) - 1)..min(i + 2, grid.len()) {
                for jj in (max(j, 1) - 1)..min(j + 2, grid[i].len()) {
                    if used[ii][jj] {
                        continue;
                    }
                    used[ii][jj] = true;
                    let row = &grid[ii];
                    if row[jj].is_numeric() {
                        // scan left
                        let mut jjj = jj;
                        while jjj > 0 && row[jjj - 1].is_numeric() {
                            jjj -= 1;
                        }
                        // read the value
                        let mut val = 0u32;
                        while jjj < row.len() && row[jjj].is_numeric() {
                            val *= 10;
                            val += row[jjj].to_digit(10).unwrap();
                            used[ii][jjj] = true;
                            jjj += 1;
                        }
                        acc += val;
                    }
                }
            }
        }
    }
    println!("{acc}");
}
fn advent03_part2() {
    println!("{:?}", env::current_dir());
    let input = fs::read_to_string("inputs/03.txt").unwrap();
    let grid: Vec<Vec<char>> = input
        .split('\n')
        .map(|line| line.trim().chars().collect())
        .collect();
    let mut used: Vec<Vec<bool>> = grid
        .iter()
        .map(|line| line.iter().map(|_| false).collect())
        .collect();
    let mut acc = 0u32;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if used[i][j] || grid[i][j] == '.' || grid[i][j].is_numeric() {
                continue;
            }
            used[i][j] = true;
            let mut vals: Vec<u32> = vec![];
            // scan around the marker
            for ii in (max(i, 1) - 1)..min(i + 2, grid.len()) {
                for jj in (max(j, 1) - 1)..min(j + 2, grid[i].len()) {
                    if used[ii][jj] {
                        continue;
                    }
                    used[ii][jj] = true;
                    let row = &grid[ii];
                    if row[jj].is_numeric() {
                        // scan left
                        let mut jjj = jj;
                        while jjj > 0 && row[jjj - 1].is_numeric() {
                            jjj -= 1;
                        }
                        // read the value
                        let mut val = 0u32;
                        while jjj < row.len() && row[jjj].is_numeric() {
                            val *= 10;
                            val += row[jjj].to_digit(10).unwrap();
                            used[ii][jjj] = true;
                            jjj += 1;
                        }
                        vals.push(val);
                    }
                }
            }
            if vals.len() == 2 {
                acc += vals.iter().product::<u32>();
            }
        }
    }
    println!("{acc}");
}

fn parse_int_set(i: &str) -> HashSet<u32> {
    i.split(" ")
        .filter(|x| x.len() > 0)
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect::<HashSet<_>>()
}

fn advent04() {
    let file = File::open("inputs/04.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line: String = String::new();
    let mut acc = 0u32;

    while {
        line.clear();
        buf_reader.read_line(&mut line).unwrap()
    } > 0
    {
        let (_, vert) = line.split_once(":").unwrap();
        let (winners, got) = vert.split_once("|").unwrap();
        let (winners2, got2) = (parse_int_set(winners), parse_int_set(got));
        let won_count = winners2.intersection(&got2).count();
        if won_count > 0 {
            acc += 1 << (won_count - 1);
        }
    }
    println!("{acc}");
}

fn advent04_part2() {
    let file = File::open("inputs/04.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line: String = String::new();

    let mut values = vec![];
    while {
        line.clear();
        buf_reader.read_line(&mut line).unwrap()
    } > 0
    {
        let (_, vert) = line.split_once(":").unwrap();
        let (winners, got) = vert.split_once("|").unwrap();
        let (winners2, got2) = (parse_int_set(winners), parse_int_set(got));
        // println!(
        //     "{winners2:?} {got2:?} {:?}",
        //     winners2.intersection(&got2).collect::<Vec<&u32>>()
        // );
        values.push(winners2.intersection(&got2).count());
    }
    for i in (0..values.len()).rev() {
        values[i] = 1 + values[(i + 1)..(i + 1 + values[i])].iter().sum::<usize>();
        // println!("{values:?}");
    }
    let acc: usize = values.iter().sum();
    println!("{acc}");
}

fn tuple3<I, T>(mut a: I) -> (T, T, T)
where
    I: Iterator<Item = T>,
{
    (a.next().unwrap(), a.next().unwrap(), a.next().unwrap())
}

fn lookup_advent05_mapping(m: &Vec<(Range<u64>, u64)>, index: u64) -> u64 {
    if m.len() == 0 || index < m[0].0.start {
        return index;
    }
    for (r, dest) in m {
        //println!("  {src} <= {index} < {}", src + len);
        if index < r.start {
            return index;
        } else if r.contains(&index) {
            return dest + (index - r.start);
        }
    }
    return index;
}

fn advent05() {
    let file = File::open("inputs/05.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line: String = String::new();

    line.clear();
    let _ = buf_reader.read_line(&mut line).unwrap();
    let (_, seeds_str) = line.trim().split_once(" ").unwrap();
    let mut seeds: Vec<u64> = seeds_str
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    line.clear();
    let _ = buf_reader.read_line(&mut line);

    let mut maps: Vec<Vec<(Range<u64>, u64)>> = vec![];
    for i in 0..7usize {
        line.clear();
        let _ = buf_reader.read_line(&mut line);

        let mut trimmed: &str;
        maps.push(vec![]);
        while {
            line.clear();
            let _ = buf_reader.read_line(&mut line);
            trimmed = line.trim();
            trimmed.len() > 0
        } {
            let number_strings = trimmed.split(" ").map(|x| x.parse::<u64>().unwrap());
            let (a, b, c) = tuple3(number_strings);
            maps[i].push((b..(b + c), a));
        }
        maps[i].sort_by_key(|(r, _)| r.start);
    }

    //seeds = vec![seeds[0]];
    for map in maps.iter() {
        seeds = seeds
            .iter()
            .map(|s| lookup_advent05_mapping(&map, *s))
            .collect();
        println!("seeds {seeds:?}");
    }
    let ans = seeds.iter().min().unwrap();
    println!("{ans:?}");
}

fn take_n(r: &Range<u64>, n: u64) -> Range<u64> {
    r.start + n..r.end
}
fn take_n_tup(r_tup: &(Range<u64>, u64), n: u64) -> (Range<u64>, u64) {
    (take_n(&r_tup.0, n), r_tup.1 + n)
}

fn apply_advent05_part2_mapping(
    map: &Vec<(Range<u64>, u64)>,
    indecies: &Vec<Range<u64>>,
) -> Vec<Range<u64>> {
    let mut out = vec![];
    let mut iter_i = map.iter();
    let mut iter_j = indecies.iter();
    let mut i = iter_i.next().unwrap().clone();
    let mut j = iter_j.next().unwrap().clone();

    loop {
        if i.0.start == j.start {
            // mapped and used area
            let n = min(i.0.end, j.end) - j.start;
            out.push(i.1..(i.1 + n));
            i = take_n_tup(&i, n);
            j = take_n(&j, n);
        } else if i.0.start < j.start {
            // mapped but unaccessed.
            let n = min(i.0.end, j.start) - i.0.start;
            i = take_n_tup(&i, n);
        } else {
            // unmapped region gets outputted as inputted.
            let n = min(j.end, i.0.start) - j.start;
            out.push(j.start..j.start + n);
            j = take_n(&j, n);
        }

        // get potentially new sections
        if i.0.start == i.0.end {
            match iter_i.next() {
                Some(ii) => i = ii.clone(),
                None => break,
            }
        }
        if j.start == j.end {
            match iter_j.next() {
                Some(jj) => j = jj.clone(),
                None => break,
            }
        }
    }

    // don't care about unaccessed mappings at the end

    // straight mappings at the end
    loop {
        if j.start != j.end {
            out.push(j.clone());
        }
        match iter_j.next() {
            Some(jj) => j = jj.clone(),
            None => break,
        }
    }

    out
}

fn advent05_part2() {
    let file = File::open("inputs/05.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line: String = String::new();

    line.clear();
    let _ = buf_reader.read_line(&mut line).unwrap();
    let (_, seeds_str) = line.trim().split_once(" ").unwrap();
    let mut seeds: Vec<Range<u64>> = seeds_str
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()[..]
        .chunks(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .collect();
    seeds.sort_by_key(|range| range.start);

    line.clear();
    let _ = buf_reader.read_line(&mut line);

    let mut maps: Vec<Vec<(Range<u64>, u64)>> = vec![];
    for i in 0..7usize {
        line.clear();
        let _ = buf_reader.read_line(&mut line);

        let mut trimmed: &str;
        maps.push(vec![]);
        while {
            line.clear();
            let _ = buf_reader.read_line(&mut line);
            trimmed = line.trim();
            trimmed.len() > 0
        } {
            let number_strings = trimmed.split(" ").map(|x| x.parse::<u64>().unwrap());
            let (a, b, c) = tuple3(number_strings);
            maps[i].push((b..(b + c), a));
        }
        maps[i].sort_by_key(|(r, _)| r.start);
    }

    //seeds = vec![seeds[0]];
    for map in maps.iter() {
        seeds = apply_advent05_part2_mapping(map, &seeds);
        seeds.sort_by_key(|s| s.start);
        //println!("seeds {seeds:?}");
    }
    let ans = seeds.iter().min_by_key(|s| s.start).unwrap().start;
    println!("{ans:?}");
}

fn advent06() {
    let file = File::open("inputs/06.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line: String = String::new();

    line.clear();
    let _ = buf_reader.read_line(&mut line).unwrap();
    let (_, times) = line.split_once(":").unwrap();
    let times: Vec<u64> = times
        .split(" ")
        .map(|t| t.trim())
        .filter(|t| t.len() > 0)
        .map(|t| t.parse().unwrap())
        .collect();

    line.clear();
    let _ = buf_reader.read_line(&mut line).unwrap();
    let (_, distances) = line.split_once(":").unwrap();
    let distances: Vec<u64> = distances
        .split(" ")
        .map(|t| t.trim())
        .filter(|t| t.len() > 0)
        .map(|t| t.parse().unwrap())
        .collect();

    let mut acc = 1;
    for (race_duration, record) in zip(times, distances) {
        let win_possibilities: u64 = (0..race_duration)
            .filter(|button_duration| {
                let distance = (race_duration - button_duration) * button_duration;
                distance > record
            })
            .map(|_| 1)
            .sum();
        acc *= win_possibilities;
    }
    println!("{acc:?}");
}

fn advent06_part2() {
    let file = File::open("inputs/06.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line: String = String::new();

    line.clear();
    let _ = buf_reader.read_line(&mut line).unwrap();
    let (_, race_duration) = line.split_once(":").unwrap();
    let race_duration: u64 = race_duration
        .split(" ")
        .map(|t| t.trim().to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();

    line.clear();
    let _ = buf_reader.read_line(&mut line).unwrap();
    let (_, record) = line.split_once(":").unwrap();
    let record: u64 = record
        .split(" ")
        .map(|t| t.trim().to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();

    // This could be optimized by solving a quadratic equation but my input isn't big enough.
    let mut acc = 1;
    let win_possibilities: u64 = (0..race_duration)
        .filter(|button_duration| {
            let distance = (race_duration - button_duration) * button_duration;
            distance > record
        })
        .map(|_| 1)
        .sum();
    acc *= win_possibilities;
    println!("{acc:?}");
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct HandBid {
    hand: Vec<u8>, // card ranks mapped onto integers, higher is better.
    bid: u32,
}
fn advent07_hand_type(hand: &Vec<u8>) -> u8 {
    // outputs an ordinal for the hand type. higher is better.
    let c: Counter<&u8, u8> = Counter::from_iter(hand.iter());

    let counts: Vec<u8> = c
        .most_common_ordered()
        .into_iter()
        .map(|(_, count)| count)
        .collect();
    if counts[0] >= 5 {
        10
    } else if counts[0] >= 4 {
        9
    } else if counts[0] >= 3 && counts[1] >= 2 {
        8
    } else if counts[0] >= 3 {
        7
    } else if counts[0] >= 2 && counts[1] >= 2 {
        6
    } else if counts[0] >= 2 {
        5
    } else {
        4
    }
}
fn advent07() {
    let file = File::open("inputs/07.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line: String = String::new();

    let mut hand_bids: Vec<HandBid> = vec![];
    while {
        line.clear();
        buf_reader.read_line(&mut line).unwrap()
    } > 0
    {
        let (hand, bid) = line.trim().split_once(" ").unwrap();
        let hand = hand
            .chars()
            .map(|c| {
                "AKQJT98765432"
                    .chars()
                    .rev()
                    .position(|c2| c2 == c)
                    .unwrap() as u8
            })
            .collect();
        hand_bids.push(HandBid {
            hand: hand,
            bid: bid.parse().unwrap(),
        });
    }

    hand_bids.sort_by_key(|hb| {
        let hand_type = advent07_hand_type(&hb.hand);
        (hand_type, hb.hand.clone()) // I wonder how this clone could be avoided.
    });
    //println!("{hand_bids:?}");
    let ans: u32 = hand_bids
        .iter()
        .enumerate()
        .map(|(i, x)| (i + 1) as u32 * x.bid)
        .sum();
    println!("{ans:?}");
}

fn advent07_part2_hand_type(hand: &Vec<u8>) -> u8 {
    // outputs an ordinal for the hand type. higher is better.
    let mut c: Counter<&u8, u8> = Counter::from_iter(hand.iter());
    let joke_count = c.remove(&0u8).unwrap_or(0);

    let mut counts: Vec<u8> = c
        .most_common_ordered()
        .into_iter()
        .map(|(_, count)| count)
        .collect();
    if counts.len() == 0 {
        counts.push(0); // It's all fucking jokers lol
    }
    counts[0] += joke_count;
    if counts[0] >= 5 {
        10
    } else if counts[0] >= 4 {
        9
    } else if counts[0] >= 3 && counts[1] >= 2 {
        8
    } else if counts[0] >= 3 {
        7
    } else if counts[0] >= 2 && counts[1] >= 2 {
        6
    } else if counts[0] >= 2 {
        5
    } else {
        4
    }
}
fn advent07_part2() {
    let file = File::open("inputs/07.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line: String = String::new();

    let mut hand_bids: Vec<HandBid> = vec![];
    while {
        line.clear();
        buf_reader.read_line(&mut line).unwrap()
    } > 0
    {
        let (hand, bid) = line.trim().split_once(" ").unwrap();
        let hand = hand
            .chars()
            .map(|c| {
                "AKQJT98765432J"
                    .chars()
                    .rev()
                    .position(|c2| c2 == c)
                    .unwrap() as u8
            })
            .collect();
        hand_bids.push(HandBid {
            hand: hand,
            bid: bid.parse().unwrap(),
        });
    }

    hand_bids.sort_by_key(|hb| {
        let hand_type = advent07_part2_hand_type(&hb.hand);
        (hand_type, hb.hand.clone()) // I wonder how this clone could be avoided.
    });
    //println!("{hand_bids:?}");
    let ans: u32 = hand_bids
        .iter()
        .enumerate()
        .map(|(i, x)| (i + 1) as u32 * x.bid)
        .sum();
    println!("{ans:?}");
}

fn advent08() {
    let file = File::open("inputs/08.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line: String = String::new();

    line.clear();
    buf_reader.read_line(&mut line).unwrap();
    let instructions: Vec<usize> = line.trim().chars().map(|c| "LR".find(c).unwrap()).collect();
    line.clear();
    buf_reader.read_line(&mut line).unwrap();

    let mut branches: HashMap<&str, [&str; 2]> = HashMap::new();
    let r = Regex::new(r"(...) = \((...), (...)\)").unwrap();
    let mut rest: String = "".to_string();
    let _ = buf_reader.read_to_string(&mut rest);
    for (_, [src, l, r]) in r.captures_iter(rest.as_str()).map(|c| c.extract()) {
        branches.insert(src, [l, r]);
    }

    let mut loc = "AAA";
    for i in 0..999999999 {
        if loc == "ZZZ" {
            println!("{i}");
            break;
        }
        loc = branches.get(loc).unwrap()[instructions[i % instructions.len()]];
    }
}

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}
fn lcm(n: u64, m: u64) -> u64 {
    n * m / gcd(n, m)
}
fn advent08_part2() {
    let file = File::open("inputs/08.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line: String = String::new();

    line.clear();
    buf_reader.read_line(&mut line).unwrap();
    let instructions: Vec<usize> = line.trim().chars().map(|c| "LR".find(c).unwrap()).collect();
    line.clear();
    buf_reader.read_line(&mut line).unwrap();

    let mut branches: HashMap<&str, [&str; 2]> = HashMap::new();
    let r = Regex::new(r"(...) = \((...), (...)\)").unwrap();
    let mut rest: String = "".to_string();
    let _ = buf_reader.read_to_string(&mut rest);
    for (_, [src, l, r]) in r.captures_iter(rest.as_str()).map(|c| c.extract()) {
        branches.insert(src, [l, r]);
    }

    // confirm that we end up in cycle land.
    // let mut fringe: Vec<&str> = branches
    //     .keys()
    //     .map(|k| *k)
    //     .filter(|k| k.chars().nth(2).unwrap() == 'A')
    //     .collect();
    // for i in 0..99999999999 {
    //     if i % 1000000 == 0 {
    //         dbg!(i);
    //     }
    //     if fringe.iter().all(|f| f.chars().nth(2).unwrap() == 'Z') {
    //         println!("{i}");
    //         break;
    //     }
    //     let instruction = instructions[i % instructions.len()];
    //     fringe = fringe
    //         .iter()
    //         .map(|loc| branches.get(loc).unwrap()[instruction])
    //         .collect();
    // }

    #[derive(Debug, Hash, PartialEq, Eq)]
    struct Cycle {
        len: u64,
        z_times: Vec<u64>, // z_times.all(|t| we're at a 'Z' location at n*len+t)
    }
    let starts: Vec<&str> = branches
        .keys()
        .map(|k| *k)
        .filter(|k| k.chars().nth(2).unwrap() == 'A')
        .collect();
    // let starts = vec!["AAA"];
    let cycles: Vec<Cycle> = starts
        .iter()
        .map(|s| {
            let mut visited: HashMap<_, u64> = HashMap::new();
            let mut finishes: Vec<u64> = vec![];
            let mut loc = *s;
            for i in 0..(branches.len() * instructions.len()) as u64 {
                let instruction_index = i as usize % instructions.len();
                if let Some(prev_i) = visited.insert((instruction_index, loc), i) {
                    dbg!(prev_i);
                    return Cycle {
                        len: i - prev_i,
                        z_times: finishes.into_iter().filter(|f| *f > prev_i).collect(),
                    };
                }

                loc = branches.get(loc).unwrap()[instructions[instruction_index]];
                if loc.chars().nth(2).unwrap() == 'Z' {
                    finishes.push(i);
                }
            }
            unreachable!();
        })
        .collect();
    // dbg!(starts);
    assert!(cycles.iter().all(|c| c.z_times.len() == 1));
    assert!(cycles.iter().all(|c| c.z_times[0] == c.len - 1));
    let ans = cycles.iter().map(|c| c.len).reduce(lcm).unwrap(); // - 1;

    dbg!(ans);
    // azazaz
    // aazaaz
    dbg!(cycles);
}

fn parse_int_list(line: &str) -> Vec<i64> {
    line.split(" ")
        .filter(|x| x.len() > 0)
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

fn advent09() {
    let input = fs::read("inputs/09.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    let input = input.trim();
    let lines: std::str::Split<'_, &str> = input.split("\n");
    let mut sequences: Vec<Vec<i64>> = lines.map(parse_int_list).collect();

    // This is for part2. wasn't worth to copy paste the code.
    // Comment it out if you want part1.
    for s in &mut sequences {
        s.reverse();
    }

    fn extrapolate(seq: &Vec<i64>) -> i64 {
        if seq.iter().all(|x| *x == 0) {
            return 0;
        }
        let diffs = &seq
            .iter()
            .tuple_windows::<(_, _)>()
            .map(|(a, b)| b - a)
            .collect();
        seq[seq.len() - 1] + extrapolate(diffs)
    }
    let ans: i64 = sequences.iter().map(extrapolate).sum();
    dbg!(ans);
}

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
        for (col, &x) in line.iter().enumerate() {
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

fn advent11() {
    let input = fs::read("inputs/11.txt").unwrap();
    let mut input = input;
    input.retain(|c| *c != b'\r');
    let lines: Vec<Vec<u8>> = input
        .split(|c| *c == b'\n')
        .map(|line| line.into())
        .collect();

    let mut galaxies: Vec<Vec<usize>> = lines
        .iter()
        .map(|line| {
            line.iter()
                .enumerate()
                .filter_map(|(i, x)| if *x == b'#' { Some(i as usize) } else { None })
                .collect()
        })
        .collect();
    // dbg!();

    fn expand_vertically(galaxies: &mut Vec<Vec<usize>>) {
        for i in (0..galaxies.len()).rev() {
            if galaxies[i].len() == 0 {
                galaxies.insert(i, vec![]);
            }
        }
    }
    fn transpose(galaxies: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let new_row_count = *galaxies
            .iter()
            .map(|row| row.iter().max().unwrap_or(&0))
            .max()
            .unwrap()
            + 1;
        let mut out = (0..new_row_count).map(|_| vec![]).collect_vec();
        for (i, row) in galaxies.iter().enumerate() {
            for &j in row.iter() {
                out[j as usize].push(i);
            }
        }
        for row in out.iter_mut() {
            row.sort();
        }
        out
    }

    expand_vertically(&mut galaxies);
    galaxies = transpose(galaxies);
    expand_vertically(&mut galaxies);
    galaxies = transpose(galaxies);
    // for row in galaxies.iter() {
    //     println!("{row:?}");
    // }

    let coords = galaxies
        .iter()
        .enumerate()
        .map(|(row, cols)| cols.iter().map(|&col| (row, col)).collect_vec())
        .flatten()
        .collect_vec();
    // dbg!(coords);
    let mut answer_part1 = 0;
    fn distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
        max(a.0, b.0) - min(a.0, b.0) + max(a.1, b.1) - min(a.1, b.1)
    }
    for (i, a) in coords.iter().enumerate() {
        for b in coords[i + 1..].iter() {
            answer_part1 += distance(a, b);
        }
    }
    dbg!(answer_part1);
}

fn advent11_part2() {
    let input = fs::read("inputs/11.txt").unwrap();
    let mut input = input;
    input.retain(|c| *c != b'\r');
    let lines: Vec<Vec<u8>> = input
        .split(|c| *c == b'\n')
        .map(|line| line.into())
        .collect();

    let mut galaxies: Vec<Vec<usize>> = lines
        .iter()
        .map(|line| {
            line.iter()
                .enumerate()
                .filter_map(|(i, x)| if *x == b'#' { Some(i as usize) } else { None })
                .collect()
        })
        .collect();

    let mut coords = galaxies
        .iter()
        .enumerate()
        .map(|(row, cols)| cols.iter().map(|&col| (row, col)).collect_vec())
        .flatten()
        .collect_vec();

    fn expand_rows(coords: &mut Vec<(usize, usize)>) {
        let mut total_new_space = 0usize;
        coords.sort();
        let expansion_factor = 1000000 - 1usize;
        let max_row = coords.iter().map(|(a, b)| *a).max().unwrap();
        let mut space_iter = 0..=max_row + 1;
        let mut coo_iter = coords.iter_mut();
        let mut row = space_iter.next().unwrap();
        let mut coo = coo_iter.next().unwrap();

        // would be neat if something like this was possible:
        // while let (Some(row2), Some(coo2)) = (row, coo) {
        loop {
            if row < coo.0 {
                total_new_space += expansion_factor;
                row = space_iter.next().unwrap();
            } else if row == coo.0 {
                row = space_iter.next().unwrap();
            } else {
                coo.0 += total_new_space;
                if let Some(coo2) = coo_iter.next() {
                    coo = coo2;
                } else {
                    break;
                }
            }
        }
    }

    fn transpose(coords: &mut Vec<(usize, usize)>) {
        for (a, b) in coords.iter_mut() {
            std::mem::swap(a, b); //(a, b) = (b, a);
        }
        coords.sort();
    }

    expand_rows(&mut coords);
    transpose(&mut coords);
    expand_rows(&mut coords);
    transpose(&mut coords);
    // println!("{coords:?}");

    let mut answer_part2 = 0;
    fn distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
        max(a.0, b.0) - min(a.0, b.0) + max(a.1, b.1) - min(a.1, b.1)
    }
    for (i, a) in coords.iter().enumerate() {
        for b in coords[i + 1..].iter() {
            answer_part2 += distance(a, b);
        }
    }
    dbg!(answer_part2);
}

#[memoize]
fn solve_12(art: String, mut nono: Vec<u8>) -> u64 {
    // println!("yay {art:?} {nono:?}");
    if art == "" {
        // println!("yee -> {}", if nono.len() == 0 { 1 } else { 0 });
        return if nono.len() == 0 { 1 } else { 0 };
    }
    let mut rest = &art[1..];
    match &art[0..1] {
        "?" => {
            // println!(
            //     "branch result: {art:?} {nono:?} -> {} {}",
            //     solve_12(format!("#{rest}"), nono.clone()),
            //     solve_12(format!(".{rest}"), nono.clone())
            // );
            let ans =
                solve_12(format!("#{rest}"), nono.clone()) + solve_12(format!(".{rest}"), nono);
            ans
        }
        "." => solve_12(rest.to_owned(), nono),
        "#" => {
            if nono.len() == 0 {
                // println!("nooope A");
                return 0;
            }
            let block_len = nono.remove(0);
            assert_ne!(block_len, 0);
            if block_len as usize > art.len()
                    || art[..(block_len as usize)].chars().any(|c| c == '.') // the block must support being all "#"
                    || ((block_len as usize) + 1  < art.len() && &art[block_len as usize..][..1] == "#")
            {
                // println!("nooope B");
                return 0;
            }
            // if block_len as usize <= rest.len() {
            rest = &rest[block_len as usize - 1..]; // take one extra for the '.' if we're not at the end.
                                                    // }
            if rest.len() > 0 {
                if &rest[0..1] == "#" {
                    return 0;
                }
                rest = &rest[1..]; // take one extra for the '.' if we're not at the end.
            }
            solve_12(rest.to_owned(), nono)
        }
        c => unimplemented!("no support for {c:?}"),
    }
}

fn solve_12_brute_force(art: String, mut nono: Vec<u8>) -> u64 {
    let free_bit_count = art.chars().filter(|c| *c == '?').count();
    (0..(1 << free_bit_count))
        .filter(|choice| {
            let mut choice = *choice;
            let mut completed_art_iter = art.chars().map(|c| {
                if c == '?' {
                    let q = choice & 1;
                    choice = choice >> 1;
                    ['#', '.'][q]
                } else {
                    c
                }
            });

            let mut nono_iter = nono.iter().map(|x| *x);
            let mut block_remaining: Option<u8> = None; //nono_iter.next();
            let mut in_block = false;
            for a in completed_art_iter {
                if !in_block {
                    match a {
                        '#' => {
                            assert!(block_remaining.is_none());
                            block_remaining = nono_iter.next();
                            if block_remaining.is_none() {
                                return false; // completed art has too many blocks
                            }
                            in_block = true;
                        }
                        '.' => {}
                        _ => unreachable!(),
                    }
                }
                // deliberately not else
                if in_block {
                    match block_remaining {
                        None => unreachable!(),
                        Some(0) => {
                            if a != '.' {
                                return false;
                            }
                            block_remaining = None;
                            in_block = false;
                        }
                        Some(b) => {
                            if a != '#' {
                                return false;
                            }
                            block_remaining = Some(b - 1);
                        }
                    }
                }
            }

            // completed art has too few blocks
            match block_remaining {
                None => {}
                Some(0) => {}
                _ => return false,
            };
            if nono_iter.next().is_some() {
                return false;
            }
            true
        })
        .count() as u64
}

fn advent12() {
    let input = read("inputs/12.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    let input = input.lines().collect_vec();

    // combination of ASCII art and 1-D nonogram hint
    let artnonos = input
        .iter()
        .map(|line| {
            let (art, nono) = line.split_once(' ').unwrap();
            let nono: Vec<u8> = nono.split(",").map(|x| x.parse().unwrap()).collect_vec();
            (art, nono)
        })
        .collect_vec();

    let mut answer1: u64 = 0;
    for (art, nono) in artnonos {
        let repeats = 5;
        // if "??#?????##" != art {
        //     continue;
        // }

        let art = std::iter::repeat(art).take(repeats).join("?");
        let nono = std::iter::repeat(nono.iter())
            .take(repeats)
            .flatten()
            .map(|x| *x)
            .collect_vec();

        // let val = solve_12_brute_force(art.to_owned(), nono.clone());
        // // if val != val2 {
        // println!("{art:?} {nono:?} -> {val:?}");
        let val2 = solve_12(art.to_owned(), nono.clone());
        println!("{art:?} {nono:?} -> {val2:?}");
        // }
        answer1 += val2;
        // break;
    }
    dbg!(answer1);
}

fn main() {
    // advent01();
    // advent02();
    // advent03();
    // advent03_part2();
    // advent04();
    // advent04_part2();
    // advent05(); // 1181555926
    // advent05_part2();
    // advent06();
    // advent06_part2();
    // advent07();
    // advent07_part2();
    // advent08();
    // advent08_part2();
    // advent09();
    // advent10();
    // advent11();
    // advent11_part2();
    advent12();
}
