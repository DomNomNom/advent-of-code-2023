use itertools::Itertools;
use std::{
    cmp::{max, min},
    collections::HashMap,
    fs::read,
};

type v3 = [u16; 3];
type aabb = (v3, v3); // inclusive, exclusive bounds

fn intersects_xy(a: &aabb, b: &aabb) -> bool {
    a.0[0] < b.1[0] && b.0[0] < a.1[0] && a.0[1] < b.1[1] && b.0[1] < a.1[1]
}
fn move_to_z(a: &mut aabb, bot_z: u16) {
    a.1[2] -= a.0[2] - bot_z;
    a.0[2] = bot_z;
}

fn main() {
    let input = read("inputs/22test.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    let input = input.lines().collect_vec();
    let mut input = input
        .into_iter()
        .map(|line| {
            let (l, r) = line.split_once("~").unwrap();
            let parse_v3 = |part: &str| -> v3 {
                part.split(",")
                    .map(|n| n.parse().unwrap())
                    .collect_vec()
                    .try_into()
                    .unwrap()
            };
            let l = parse_v3(l);
            let mut r = parse_v3(r);
            r[0] += 1;
            r[1] += 1;
            r[2] += 1;
            // let bot = min(l, r);
            // let top = max(l, r);
            assert_eq!(l, min(l, r));
            assert_eq!(r, max(l, r));
            (l, r)
        })
        .collect_vec();

    let to_letter: HashMap<aabb, char> = HashMap::from_iter(
        input
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, b)| (b, "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().nth(i).unwrap())),
    );

    for (j, down) in input.iter().enumerate() {
        for (i, up) in input.iter().enumerate() {
            if up.0[2] < down.0[2] || i == j {
                continue; // wrong direction
            }
            if intersects_xy(up, down) {
                println!("{} -upish-> {}", to_letter[down], to_letter[up]);
            }
        }
    }
    println!();

    input.sort_by_key(|brick| brick.1[2]); // lowest top-edge first
    let to_letter: HashMap<usize, char> =
        HashMap::from_iter(input.iter().enumerate().map(|(i, b)| (i, to_letter[b])));

    // make the blocks fall and build dependency graph
    let mut to_down: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut to_up: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut fallen: Vec<aabb> = vec![];
    for (i, mut up) in input.into_iter().enumerate() {
        let mut support_z: Option<u16> = None;
        println!("{:?}", fallen.iter().enumerate().rev().collect_vec());
        for (j, down) in fallen.iter().enumerate().rev() {
            if let Some(z) = support_z {
                if down.1[2] > z {
                    unreachable!("impossibru");
                }
                if down.1[2] < z {
                    break;
                }
            }
            if intersects_xy(&up, down) {
                support_z = Some(down.1[2]);
                (*to_down.entry(i).or_default()).push(j);

                println!(
                    "{} -upaa-> {} at z={}",
                    to_letter[&j],
                    to_letter[&i],
                    support_z.unwrap()
                );
            }
        }
        if let Some(z) = support_z {
            move_to_z(&mut up, z);
            println!("on block: {} z={}..{}", to_letter[&i], up.0[2], up.1[2]);
        } else {
            move_to_z(&mut up, 1);
            assert_eq!(to_down.insert(i, vec![]), None);
            println!("on ground: {}  z={}..{}", to_letter[&i], up.0[2], up.1[2]);
        }
        fallen.push(up);
        fallen.sort_by_key(|brick| brick.1[2]); // lowest top-edge first
    }

    for (up, downs) in to_down.iter() {
        for down in downs {
            (*to_up.entry(*down).or_default()).push(*up);
        }
    }
    for i in 0..to_down.len() {
        if !to_up.contains_key(&i) {
            to_up.insert(i, vec![]);
        }
    }

    for (i, ups) in to_up.iter().sorted_by_key(|(i, _)| to_letter[i]) {
        println!(
            "{} -up-> {:?}",
            to_letter[i],
            ups.iter().map(|up| to_letter[up]).collect_vec()
        )
    }
    for (i, downs) in to_down.iter().sorted_by_key(|(i, _)| to_letter[i]) {
        println!(
            "{} -down-> {:?}",
            to_letter[i],
            downs.iter().map(|down| to_letter[down]).collect_vec()
        )
    }
    let answer1 = to_up
        .iter()
        .filter(|&(_k, ups)| ups.iter().all(|up| to_down[up].len() > 1))
        .count();
    dbg!(answer1);
    // for brick in bricks {}
}
