use chumsky::{prelude::*, text::ident};
use itertools::Itertools;
use std::{
    collections::{hash_map, HashMap, VecDeque},
    fs::read,
    process::Output,
};

type OutWiring = Vec<(String, usize)>; // the usize is the index into Nand input cache

#[derive(Clone, Debug)]
struct FlipFlop {
    state: bool, // true => ON
}
#[derive(Clone, Debug)]
struct Nand {
    input_cache: Vec<bool>, // true => HIGH
}
// #[derive(Clone, Debug)]
// struct Passthrough {}

#[derive(Clone, Debug)]
enum Gate {
    Nand(Nand),
    FlipFlop(FlipFlop),
    Passthrough,
}
// #[derive(Clone, Debug)]
// struct WiredGate {
//     gate: Gate,
//     wires: OutWiring,
// }

fn parser(
) -> impl Parser<char, (HashMap<String, Gate>, HashMap<String, OutWiring>), Error = Simple<char>> {
    let wires = just(" -> ")
        .ignore_then(ident().separated_by(just(", ")))
        .map(|names| names.into_iter().map(|name| (name, 0usize)).collect_vec());

    let gate1 =
        just("%")
            .ignore_then(ident())
            .then(wires)
            .map(|(name, wires): (String, OutWiring)| {
                (name, Gate::FlipFlop(FlipFlop { state: false }), wires)
            });
    let gate2 = just("&")
        .ignore_then(ident())
        .then(wires)
        .map(|(name, wires)| {
            (
                name,
                Gate::Nand(Nand {
                    input_cache: vec![],
                }),
                wires,
            )
        });
    let gate3 = ident()
        .then(wires)
        .map(|(name, wires)| (name, Gate::Passthrough, wires));
    let line = gate1.or(gate2).or(gate3);
    let lines = line.padded().repeated();
    lines.map(|lines| {
        let mut gates: HashMap<String, Gate> = HashMap::new();
        let mut network: HashMap<String, OutWiring> = HashMap::new();
        let mut keys: Vec<String> = Vec::new();
        for (name, gate, wires) in lines {
            gates.insert(name.to_string(), gate);
            network.insert(name.to_string(), wires);
            keys.push(name);
        }

        // initialize the wiring and NAND input caches
        for name in keys {
            for (target_name, index) in network.get_mut(&name).unwrap().iter_mut() {
                *index = match &mut gates.get_mut(target_name).unwrap() {
                    Gate::Nand(nand) => {
                        nand.input_cache.push(false);
                        nand.input_cache.len() - 1usize
                    }
                    _ => 0usize,
                }
            }
        }

        return (gates, network);
    })
}

fn pulse(
    gates: &mut HashMap<String, Gate>,
    network: &HashMap<String, OutWiring>,
    target: &(String, usize),
    high: bool,
) -> u64 {
    let mut q = VecDeque::from(vec![target]);
    let mut count_high = 0u64;
    let mut count_low = 0u64;
    while let Some((name, input_index)) = q.pop_front() {
        // apply state change
        let high = match &mut gates.get_mut(name).unwrap() {
            Gate::Nand(nand) => {
                nand.input_cache[*input_index] = high;
                nand.input_cache.iter().all(|state| *state)
            }
            Gate::FlipFlop(ff) => {
                if high {
                    return 0;
                } else {
                    ff.state = !ff.state;
                    ff.state
                }
            }
            Gate::Passthrough => high,
        };

        let foo = &network[name];
        if high {
            count_high += foo.len() as u64;
        } else {
            count_low += foo.len() as u64;
        }
        for target in foo {
            println!(
                "{name} -{}-> {}[{}]",
                if high { "high" } else { "low" },
                target.0,
                target.1
            );

            q.push_back(target);
        }
    }
    count_high * count_low
}

fn main() {
    let input = read("inputs/20test.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    // let input = input.lines().collect_vec();
    let (mut gates, network) = parser().parse(input).unwrap();
    // dbg!(gates);
    // dbg!(network);
    let answer1 = pulse(&mut gates, &network, &("broadcaster".to_string(), 0), false);
}
