fn part1(text: &str) -> usize {
    let mut numbers = vec![];
    let mut operators = None;

    for line in text.lines() {
        let mut items = line.split(' ').filter(|str| str != &"").peekable();
        if items.peek().unwrap().parse::<usize>().is_ok() {
            numbers.push(
                items
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            );
        } else {
            operators = Some(items.collect::<Vec<_>>());
            break;
        }
    }

    let operators = operators.unwrap();

    for number_list in &numbers {
        assert_eq!(operators.len(), number_list.len());
    }

    let mut grand_total = 0;
    for (index, op) in operators.into_iter().enumerate() {
        match op {
            "+" => grand_total += numbers.iter().map(|list| list[index]).sum::<usize>(),
            "*" => {
                grand_total += numbers
                    .iter()
                    .map(|list| list[index])
                    .fold(1, |prod, n| prod * n)
            }
            _ => unreachable!(),
        }
    }

    return grand_total;
}

fn part2(_text: &str) -> usize {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day6.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const TEXT1: &str = indoc! {"
        123 328  51 64 
        45 64  387 23 
        6 98  215 314
        *   +   *   +  
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(crate::part1(TEXT1), 4277556);
    }

    // #[rstest::rstest]
    // #[case(TEXT1, 0)]
    // fn part1_given_examples(#[case] text: &str, #[case] expected: usize) {
    //     assert_eq!(crate::part1(text), expected);
    // }
}
