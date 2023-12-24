use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

type Output = usize;

// TODO: I can't believe it took me 10 minutes to realize it's x-mas...
#[derive(Debug)]
enum Category {
    X, // Extremely cool looking
    M, // Musical (it makes a noise when you hit it)
    A, // Aerodynamic
    S, // Shiny
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

fn part1(text: &str) -> Output {
    let (first_half, second_half) = text.split_once("\n\n").unwrap();
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

fn part2(_text: &str) -> Output {
    0
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
