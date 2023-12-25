use chumsky::{prelude::*, text::ident};
use itertools::Itertools;
use std::{
    cmp::{max, min},
    collections::{HashMap, VecDeque},
    fs::read,
    ops::Range,
};

type Part = [u16; 4];
#[derive(Clone, Debug)]
enum Comparison {
    LT,
    GT,
}

#[derive(Clone, Debug)]
enum JumpLabel {
    WorkflowIndex(usize),
    Accept,
    Reject,
}

#[derive(Clone, Debug)]
struct CondJump {
    part_member: usize,
    comp: Comparison,
    threshold: u16,
    label: JumpLabel,
}

#[derive(Clone, Debug)]
struct Workflow {
    conditions: Vec<CondJump>,
    default_label: JumpLabel,
}

fn parser() -> impl Parser<char, ((Vec<Workflow>, usize), Vec<Part>), Error = Simple<char>> {
    let workflow_name = text::ident().map(|s: String| s);
    let cond_jump = (one_of("xmas").map(|c| match c {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's' => 3,
        _ => unreachable!(),
    }))
    .then(
        just('<')
            .to(Comparison::LT)
            .or(just('>').to(Comparison::GT)),
    )
    .then(text::int(10).map(|i: String| i.parse::<u16>().unwrap()))
    .then_ignore(just(':'))
    .then(text::ident());
    let workflow_line = workflow_name
        .then_ignore(just("{"))
        .then(cond_jump.separated_by(just(',')))
        .then_ignore(just(','))
        .then(ident())
        .then_ignore(just("}"));
    // .delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}')));

    // let foo = workflow_line.repeated().map(|fs, _| {});

    let part_assignment = one_of("xmas")
        .ignore_then(just('='))
        .ignore_then(text::int(10))
        .map(|i| i.parse::<u16>().unwrap());
    let parts_line = part_assignment
        .separated_by(just(','))
        .exactly(4)
        .delimited_by(just('{'), just('}'))
        .map(|xmas| xmas.try_into().unwrap());

    workflow_line
        .separated_by(text::newline())
        .then_ignore(text::newline())
        .then(parts_line.padded().repeated())
        .map(|(mut workflows, parts)| {
            // let workflows =
            // workflows.sort_by_key(|tup| tup.0 .0 == "in"); // ensure workflow 0 is "in"
            let label_to_index = HashMap::<String, usize>::from_iter(
                workflows
                    .iter()
                    .enumerate()
                    .map(|(i, tup)| (tup.0 .0.clone(), i)),
            );
            let get_jump_label = |label: &str| match label {
                "A" => JumpLabel::Accept,
                "R" => JumpLabel::Reject,
                l => JumpLabel::WorkflowIndex(label_to_index[l]),
            };
            let start = workflows
                .iter()
                .position(|tup| tup.0 .0 == "in")
                .expect("want an 'in' starting point");
            (
                (
                    workflows
                        .into_iter()
                        .map(|((_name, conditions), default_label)| Workflow {
                            conditions: conditions
                                .into_iter()
                                .map(|(((part_member, comp), threshold), label)| CondJump {
                                    part_member,
                                    comp,
                                    threshold,
                                    label: get_jump_label(label.as_str()),
                                })
                                .collect_vec(),
                            default_label: get_jump_label(default_label.as_str()),
                        })
                        .collect_vec(),
                    start,
                ),
                parts,
            )
        })
}

fn optional_jump<'a>(c: &'a CondJump, part: &Part) -> Option<&'a JumpLabel> {
    let val = part[c.part_member];
    let passed = match c.comp {
        Comparison::LT => val < c.threshold,
        Comparison::GT => val > c.threshold,
    };
    if passed {
        Some(&c.label)
    } else {
        None
    }
}
fn run_workflow<'a>(w: &'a Workflow, part: &Part) -> &'a JumpLabel {
    for c in w.conditions.iter() {
        if let Some(label) = optional_jump(c, part) {
            return label;
        }
    }
    return &w.default_label;
}

// check for loops and potential DAG path explosions.
// fn recurse(workflows: &Vec<Workflow>, label: &JumpLabel) -> u64 {
//     match label {
//         JumpLabel::Accept => 1,
//         JumpLabel::Reject => 0,
//         JumpLabel::WorkflowIndex(i) => part2(workflows, *i),
//     }
// }
// fn part2(workflows: &Vec<Workflow>, start_workflow: usize) -> u64 {
//     let mut answer2 = 0u64;
//     let w = &workflows[start_workflow];
//     for cond in w.conditions.iter() {
//         answer2 += recurse(workflows, &cond.label);
//     }
//     answer2 += recurse(workflows, &w.default_label);
//     dbg!(answer2);
//     answer2
// }

fn recurse(workflows: &Vec<Workflow>, label: &JumpLabel, range: PartRange) -> u64 {
    match label {
        JumpLabel::Accept => range
            .iter()
            .map(|r| (r.end - r.start) as u64)
            .product::<u64>(),
        JumpLabel::Reject => 0,
        JumpLabel::WorkflowIndex(i) => part2(workflows, *i, range),
    }
}

type PartRange = [Range<u16>; 4];

fn part2(workflows: &Vec<Workflow>, start_workflow: usize, mut range: PartRange) -> u64 {
    let mut answer2 = 0u64;
    let w = &workflows[start_workflow];
    for cond in w.conditions.iter() {
        let m = cond.part_member;
        let mut rang2 = range.clone();
        match cond.comp {
            Comparison::LT => {
                rang2[m].end = min(rang2[m].end, cond.threshold);
                range[m].start = max(range[m].start, cond.threshold);
            }
            Comparison::GT => {
                rang2[m].start = max(rang2[m].start, cond.threshold + 1);
                range[m].end = min(range[m].end, cond.threshold + 1);
            }
        }
        range[m].start = min(range[m].end, range[m].start);
        rang2[m].start = min(rang2[m].end, rang2[m].start);
        answer2 += recurse(workflows, &cond.label, rang2);
    }
    answer2 += recurse(workflows, &w.default_label, range);
    answer2
}

fn main() {
    let input = read("inputs/19.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    let ((workflows, start_workflow), parts) = parser().parse(input).unwrap();
    {
        let mut answer1 = 0u64;
        for part in parts.iter() {
            let mut w = start_workflow;
            loop {
                let label = run_workflow(&workflows[w], part);
                // println!("{w:?} => {label:?}");
                match label {
                    JumpLabel::WorkflowIndex(i) => w = *i,
                    JumpLabel::Accept => {
                        answer1 += part.iter().map(|m| *m as u64).sum::<u64>();
                        break;
                    }
                    JumpLabel::Reject => {
                        break;
                    }
                }
            }
            // println!("  ");
        }
        dbg!(answer1);
    }
    println!("  ");

    let answer2 = part2(
        &workflows,
        start_workflow,
        [1..4001, 1..4001, 1..4001, 1..4001],
    );
    dbg!(answer2);
}
