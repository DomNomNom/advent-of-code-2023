use chumsky::{prelude::*, text::ident};
use itertools::Itertools;
use std::{collections::HashMap, fmt, fs::read};

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

fn parser() -> impl Parser<char, (Vec<Workflow>, Vec<Part>), Error = Simple<char>> {
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
        .separated_by(just("\r\n"))
        .then_ignore(just("\r\n\r\n"))
        .then(parts_line.padded().repeated())
        .map(|(workflows, parts)| {
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
                parts,
            )
        })
}

fn main() {
    let input = read("inputs/19test.txt").unwrap();
    let input = String::from_utf8(input).unwrap();
    let (workflows, parts) = parser().parse(input).unwrap();
    dbg!((workflows, parts));
}
