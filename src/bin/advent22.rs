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

    input.sort_by_key(|brick| brick.1[2]); // lowest top-edge first

    // make the blocks fall and build dependency graph
    let mut to_down: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut to_up: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut fallen: Vec<aabb> = vec![];
    for (i, mut falling) in input.into_iter().enumerate() {
        let mut support_z: Option<u16> = None;
        for (j, support) in fallen.iter().enumerate().rev() {
            if let Some(z) = support_z {
                if support.1[2] < z {
                    break;
                }
            }
            if intersects_xy(&falling, support) {
                support_z = Some(support.1[2]);
                (*to_down.entry(i).or_default()).push(j);
            }
        }
        if let Some(z) = support_z {
            move_to_z(&mut falling, z);
        } else {
            move_to_z(&mut falling, 1);
            fallen.push(falling);
            assert_eq!(to_down.insert(i, vec![]), None);
        }
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
    let answer1 = to_up
        .iter()
        .filter(|&(k, ups)| ups.iter().all(|up| to_down[up].len() > 1))
        .count();
    dbg!(answer1);
    // for brick in bricks {}
}
