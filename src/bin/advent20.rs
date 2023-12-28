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
type GatesAndWires = (HashMap<String, Gate>, HashMap<String, OutWiring>);

fn parser(
) -> impl Parser<char, GatesAndWires, Error = Simple<char>> {
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

        // Creates dangling "output" nodes
        let mut new_nodes = vec![];
        for (_src, targets) in network.iter() {
            for (target_name, _) in targets {
                if !gates.contains_key(target_name) {
                    // assert_eq!(target_name, "output"); // This should be special
                    println!("creating dangling output node: {target_name:?}");
                    new_nodes.push(target_name.to_string());
                }
            }
        }
        for name in new_nodes {
            gates.insert(name.to_string(), Gate::Passthrough);
            network.insert(name.to_string(), vec![]);
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
) -> (u64, u64) {
    let mut q = VecDeque::from(vec![(target, high)]);
    let mut high_count = 0u64;
    let mut low_count = 0u64;
    while let Some(((name, input_index), high)) = q.pop_front() {
        // apply state change
        let high = match &mut gates.get_mut(name).unwrap() {
            Gate::Nand(nand) => {
                nand.input_cache[*input_index] = high;
                // println!("nand update! {:?} but was {}", nand.input_cache, high);
                !nand.input_cache.iter().all(|state| *state)
            }
            Gate::FlipFlop(ff) => {
                if high {
                    continue;
                } else {
                    ff.state = !ff.state;
                    ff.state
                }
            }
            Gate::Passthrough => high,
        };

        let foo = &network[name];
        if high {
            high_count += foo.len() as u64;
        } else {
            low_count += foo.len() as u64;
        }
        for target in foo {
            // println!(
            //     "{name} -{}-> {}",
            //     if high { "high" } else { "low" },
            //     target.0,
            //     // target.1
            // );

            q.push_front((target, high));
        }
    }
    // dbg!(low_count);
    // dbg!(high_count);
    (low_count, high_count)
}

fn separate_graph(gates: )

fn main() {
    let input = read("inputs/20.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    // let input = input.lines().collect_vec();
    let (mut gates, mut network) = parser().parse(input).unwrap();

    gates.insert("button".to_string(), Gate::Passthrough);
    network.insert("button".to_string(), vec![("broadcaster".to_string(), 0)]);

    let original_gates = gates.clone();

    // dbg!(gates);
    // dbg!(network);
    let mut low_count = 0u64;
    let mut high_count = 0u64;
    for _ in 0..1000 {
        let (a, b) = pulse(&mut gates, &network, &("button".to_string(), 0), false);
        low_count += a;
        high_count += b;
    }
    let answer1 = low_count * high_count;
    dbg!(low_count);
    dbg!(high_count);
    dbg!(answer1);

    // part2
    gates = original_gates;
    if !gates.contains_key("rx") || true {
        println!("not doing part2");
        return;
    }
    gates.insert(
        "rx".to_string(),
        Gate::Nand(Nand {
            input_cache: vec![true],
        }),
    );
    let mut answer2 = 0u64;
    while match gates.get("rx").unwrap() {
        Gate::Nand(nand) => nand.input_cache[0],
        _ => true,
    } {
        let _ = pulse(&mut gates, &network, &("button".to_string(), 0), false);
        answer2 += 1;
        if answer2 % 10000 == 0 {
            dbg!(answer2);
        }
    }
    dbg!(answer2);
}
