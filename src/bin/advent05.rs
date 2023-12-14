#![allow(dead_code)]
use std::{
    cmp::min,
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
};

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

fn main() {
    advent05();
    advent05_part2();
}
