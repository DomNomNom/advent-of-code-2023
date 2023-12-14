#![allow(dead_code)]
use regex::Regex;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

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

fn main() {
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
