use itertools::Itertools;
use std::{collections::HashSet, fs::read};

use vec3D::Vec3D;

type PosVel = (Vec3D, Vec3D);

// adapted from dist3D_Line_to_Line from https://web.archive.org/web/20180704020228/http://www.geomalgorithms.com/a07-_distance.html
fn approach_times(track1: &PosVel, track2: &PosVel) -> (f64, f64) {
    let u = track1.1;
    let v = track2.1;
    let w = track1.0 - track2.0;
    let a = u.dot(u); // always >= 0
    let b = u.dot(v);
    let c = v.dot(v); // always >= 0
    let d = u.dot(w);
    let e = v.dot(w);
    let big_d = a * c - b * b; // always >= 0

    // compute the line parameters of the two closest points
    if (big_d < 0.00000001) {
        println!("parallel");
        // the lines are almost parallel
        (0.0, if b > c { d / b } else { e / c }) // use the largest denominator
    } else {
        ((b * e - c * d) / big_d, (a * e - b * d) / big_d)
    }

    // // get the difference of the two closest points
    // Vector   dP = w + (sc * u) - (tc * v);  // =  L1(sc) - L2(tc)
    // return norm(dP);   // return the closest distance
}

fn main() {
    let input = read("inputs/24.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    let input = input.lines().collect_vec();
    let mut particles = input
        .into_iter()
        .map(|line| {
            let (pos, vel) = line.split_once(" @ ").unwrap();
            let pos = pos
                .split(", ")
                .map(|x| x.parse::<f64>().unwrap())
                .collect_vec();
            let vel = vel
                .split(", ")
                .map(|x| x.trim().parse::<f64>().unwrap())
                .collect_vec();
            (
                Vec3D::new(pos[0], pos[1], pos[2]),
                Vec3D::new(vel[0], vel[1], vel[2]),
            )
        })
        .collect_vec();

    // surely this won't cause problems later
    for (ref mut pos, ref mut vel) in particles.iter_mut() {
        pos.z = 0.;
        vel.z = 0.;
    }

    // sanity check - no duplicate particles
    for (i, p1) in particles.iter().enumerate() {
        for (_j, p2) in particles[0..i].iter().enumerate() {
            assert_ne!(p1, p2);
        }
    }
    // sanity check - no still particles
    for (_pos, vel) in particles.iter() {
        assert!(vel.mag() > 0.1);
    }
    for (_pos, vel) in particles.iter() {
        assert!(vel.mag() > 0.1);
    }

    let mut answer1 = 0;
    let test_region_bot = Vec3D::new(7., 7., -100.);
    let test_region_top = Vec3D::new(27., 27., 100.);
    let test_region_bot = Vec3D::new(200000000000000., 200000000000000., -100.);
    let test_region_top = Vec3D::new(400000000000000., 400000000000000., 100.);
    for (i, a) in particles.iter().enumerate() {
        for (_j, b) in particles[0..i].iter().enumerate() {
            if b.1.cross(a.1).mag() < 0.001 {
                // println!("{} {} parallel", i, _j);
                continue; // parallel paths
            }
            let (ta, tb) = approach_times(a, b);

            if ta < 0. || tb < 0. {
                // println!("{} {} in past", i, _j);
                continue;
            }

            let pa = a.0 + a.1 * ta;
            let pb = b.0 + b.1 * tb;
            assert!(pa.distance_to(pb) < 0.00001 * pa.mag());
            // println!("{} {} -> location={} {} at t={} {}", i, _j, pa, pb, ta, tb);
            if false // style
                || pa.x < test_region_bot.x
                || pa.y < test_region_bot.y
                || pa.z < test_region_bot.z
                || pa.x >= test_region_top.x
                || pa.y >= test_region_top.y
                || pa.z >= test_region_top.z
            {
                // println!("{} {} out of box at location={:?}", i, _j, pa);
                continue;
            }
            // println!("{} {} all good", i, _j);
            answer1 += 1;
        }
    }
    dbg!(answer1);
}
