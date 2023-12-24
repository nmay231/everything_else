use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Range;

use itertools::Itertools;

type Output = usize;

// TODO: I can't believe it took me 10 minutes to realize it's x-mas...
#[derive(Debug, Clone, Copy, PartialEq)]
enum Category {
    X, // Extremely cool looking
    M, // Musical (it makes a noise when you hit it)
    A, // Aerodynamic
    S, // Shiny
}

impl Category {
    fn as_index(&self) -> usize {
        match self {
            Category::X => 0,
            Category::M => 1,
            Category::A => 2,
            Category::S => 3,
        }
    }
}

#[derive(Debug)]
struct Part(usize, usize, usize, usize);

impl Part {
    fn new(x: usize, m: usize, a: usize, s: usize) -> Self {
        Self(x, m, a, s)
    }

    fn from_category(&self, cat: &Category) -> usize {
        match cat {
            Category::X => self.0,
            Category::M => self.1,
            Category::A => self.2,
            Category::S => self.3,
        }
    }
}

#[derive(Debug)]
enum WorkflowCheck<'a> {
    ChangeWorkflow(&'a str),
    CategoryCheck(Category, Ordering, usize, &'a str),
}

fn parse_workflows(first_half: &str) -> HashMap<&str, Vec<WorkflowCheck<'_>>> {
    let mut workflows = HashMap::<&str, Vec<WorkflowCheck>>::new();
    for line in first_half.lines() {
        let (workflow_name, mut line) = line[..line.len() - 1].split_once('{').unwrap();
        let mut workflow = vec![];
        while let Some((workflow_check, next_steps)) = line.split_once(',') {
            let cat = match &workflow_check[..1] {
                "x" => Category::X,
                "m" => Category::M,
                "a" => Category::A,
                "s" => Category::S,
                _ => panic!("Unexpected category for line '{line}'"),
            };
            let order = match &workflow_check[1..2] {
                "<" => Ordering::Less,
                ">" => Ordering::Greater,
                _ => panic!("Unexpected ordering for line '{line}'"),
            };
            let (amount, next_workflow) = workflow_check[2..].split_once(':').unwrap();
            // println!("{:?}", (&cat, order, amount, next_workflow, line));
            workflow.push(WorkflowCheck::CategoryCheck(
                cat,
                order,
                amount.parse().unwrap(),
                next_workflow,
            ));
            line = next_steps;
        }
        workflow.push(WorkflowCheck::ChangeWorkflow(line));
        workflows.insert(workflow_name, workflow);
    }
    workflows
}

fn part1(text: &str) -> Output {
    let (first_half, second_half) = text.split_once("\n\n").unwrap();
    let workflows = parse_workflows(first_half);

    // println!();
    // for line in first_half.lines() {
    //     let cat = &line[..line.find('{').unwrap()];
    //     println!("{cat}: {:?}", workflows.get(cat));
    // }
    // println!();

    let parts = second_half
        .lines()
        .map(|ori| {
            let line = ori;
            let line = &line[1..line.len() - 1];
            let (x, line) = line.split_once(',').unwrap();
            let (m, line) = line.split_once(',').unwrap();
            let (a, s) = line.split_once(',').unwrap();
            // println!("{:?}", (ori, x, m, a, s));
            Part::new(
                x[2..].parse().unwrap(),
                m[2..].parse().unwrap(),
                a[2..].parse().unwrap(),
                s[2..].parse().unwrap(),
            )
        })
        .collect_vec();
    // println!("{:?}", parts);
    // println!();

    let mut parts_sum = 0;
    for part in parts {
        let mut workflow_name = "in";
        while workflow_name != "R" && workflow_name != "A" {
            let workflow = workflows.get(workflow_name).unwrap();
            for check in workflow {
                match check {
                    WorkflowCheck::ChangeWorkflow(new_workflow) => {
                        workflow_name = new_workflow;
                        break;
                    }
                    WorkflowCheck::CategoryCheck(cat, order, amount, new_workflow) => {
                        if part.from_category(cat).cmp(amount) == *order {
                            workflow_name = new_workflow;
                            break;
                        }
                    }
                }
            }
        }

        if workflow_name == "A" {
            // println!("accepted: {:?}", part);
            parts_sum += part.0 + part.1 + part.2 + part.3;
        } else {
            // println!("rejected: {:?}", part);
        }
    }

    parts_sum
}

#[derive(Debug, Clone, PartialEq)]
struct CategoryConstraint(Category, Ordering, usize);

#[derive(Debug, Clone, PartialEq)]
struct PartConstraint([Range<usize>; 4]);

impl PartConstraint {
    fn count_possible_parts(&self) -> Output {
        self.0.iter().fold(1, |acc, range| acc * range.len())
    }
}

#[derive(Debug, Clone)]
enum SplitPartConstraint {
    Accept,
    Reject,
    AcceptReject(PartConstraint, PartConstraint),
}

// TODO: This struct doesn't need to exist really and split_accept_reject()
// TODO: should really be a method of PartConstraint, but I don't care rn :)
impl CategoryConstraint {
    fn split_accept_reject(&self, part_constraint: &PartConstraint) -> SplitPartConstraint {
        let index = self.0.as_index();
        match self.1 {
            Ordering::Equal => unreachable!("No direct equality constraints in this puzzle"),
            Ordering::Less => {
                if self.2 <= part_constraint.0[index].start {
                    SplitPartConstraint::Reject
                } else if part_constraint.0[index].end < self.2 {
                    SplitPartConstraint::Accept
                } else {
                    let mut accept = part_constraint.to_owned();
                    let mut reject = part_constraint.to_owned();
                    accept.0[index] = accept.0[index].start..self.2;
                    reject.0[index] = self.2..reject.0[index].end;
                    SplitPartConstraint::AcceptReject(accept, reject)
                }
            }
            Ordering::Greater => {
                if part_constraint.0[index].end <= self.2 {
                    SplitPartConstraint::Reject
                } else if self.2 < part_constraint.0[index].start {
                    SplitPartConstraint::Accept
                } else {
                    let mut accept = part_constraint.to_owned();
                    let mut reject = part_constraint.to_owned();
                    accept.0[index] = self.2 + 1..accept.0[index].end;
                    reject.0[index] = reject.0[index].start..self.2 + 1;
                    SplitPartConstraint::AcceptReject(accept, reject)
                }
            }
        }
    }
}

// impl Neg for CategoryConstraint {
//     type Output = Option<CategoryConstraint>;
//     fn neg(self) -> Self::Output {
//         match self.1 {
//             Ordering::Equal => panic!("I don't ever need equals"),
//             Ordering::Less => Some(CategoryConstraint(
//                 self.0,
//                 Ordering::Greater,
//                 self.2.checked_sub(1)?,
//             )),
//             Ordering::Greater => Some(CategoryConstraint(
//                 self.0,
//                 Ordering::Less,
//                 self.2.checked_add(1)?,
//             )),
//         }
//     }
// }

fn part2(text: &str) -> Output {
    let (first_half, _) = text.split_once("\n\n").unwrap();
    let workflows = parse_workflows(first_half);

    let mut parts = vec![("in", PartConstraint([1..4001, 1..4001, 1..4001, 1..4001]))];
    assert_eq!(4000 * 4000 * 4000 * 4000, parts[0].1.count_possible_parts());

    let mut accepted = 0;
    let mut rejected = 0;
    while let Some((workflow_name, mut part)) = parts.pop() {
        // println!("{:?}", (workflow_name, &part));
        if workflow_name == "R" {
            rejected += part.count_possible_parts();
            continue;
        } else if workflow_name == "A" {
            accepted += part.count_possible_parts();
            continue;
        }
        let workflow = workflows.get(workflow_name).unwrap();
        for check in workflow {
            match check {
                WorkflowCheck::ChangeWorkflow(next) => {
                    parts.push((next, part));
                    break;
                }
                WorkflowCheck::CategoryCheck(cat, order, amount, name_if_accept) => {
                    match CategoryConstraint(*cat, *order, *amount).split_accept_reject(&part) {
                        SplitPartConstraint::Accept => {
                            parts.push((name_if_accept, part));
                            break;
                        }
                        SplitPartConstraint::Reject => (),
                        SplitPartConstraint::AcceptReject(accept, reject) => {
                            parts.push((name_if_accept, accept));
                            part = reject;
                        }
                    }
                }
            }
        }
    }

    assert_eq!(4000 * 4000 * 4000 * 4000, accepted + rejected);

    accepted
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day19.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample() {
        assert!(true);
    }
}
