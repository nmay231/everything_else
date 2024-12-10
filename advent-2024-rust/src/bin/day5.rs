use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

type Output = usize;

// TODO: Yes, it should be a Result to allow for helpful debug info, but life is too short
fn parse(text: &str) -> Option<(HashMap<usize, Vec<usize>>, Vec<Vec<usize>>)> {
    let lines = text.trim().lines();
    let mut orderings = HashMap::<usize, Vec<usize>>::new();
    let mut page_updates = vec![];
    let mut first_block = true;
    for line in lines {
        match (first_block, line) {
            (true, "") => first_block = false,
            (false, "") => panic!("Reached two blank lines in the middle of the file"),
            (true, ordering_pair) => {
                let (a, b) = ordering_pair.split_once('|')?;
                orderings
                    .entry(a.parse().ok()?)
                    .or_default()
                    .push(b.parse().ok()?);
            }
            (false, pages) => page_updates.push(
                pages
                    .split(',')
                    .map(|page| page.parse())
                    .collect::<Result<Vec<_>, _>>()
                    .ok()?,
            ),
        }
    }

    let mut first = HashSet::new();
    let mut second = HashSet::new();
    for (a, bs) in &orderings {
        first.insert(a);
        for b in bs {
            first.insert(b);
        }
    }
    for pages in &page_updates {
        for a in pages {
            second.insert(a);
        }
    }
    // Sanity check - The phrasing of part 2 suggests that there is no ambiguity
    // of which number is before the other when you correct one of them, but I
    // couldn't stand just assuming that.
    assert_eq!(
        first, second,
        "I assume the (partial) ordering of numbers is un-ambiguous"
    );

    Some((orderings, page_updates))
}

fn part1(text: &str) -> Output {
    let (orderings, page_updates) = parse(text).expect("Error parsing sections of the file");
    let mut total = 0;

    'page_updates: for pages in page_updates {
        let mut up_to_now = vec![];
        for page in pages {
            match orderings.get(&page) {
                None => {}
                Some(must_be_after) => {
                    for before in &up_to_now {
                        if must_be_after.contains(before) {
                            continue 'page_updates;
                        }
                    }
                }
            }
            up_to_now.push(page);
        }

        assert!(up_to_now.len() & 1 == 1);
        total += up_to_now[up_to_now.len() / 2];
    }
    total
}

fn part2(text: &str) -> Output {
    let (orderings, page_updates) = parse(text).expect("Error parsing sections of the file");
    let mut total = 0;

    for mut pages in page_updates {
        let old_pages = pages.clone();
        pages.sort_by(|a, b| {
            orderings
                .get(a)
                .and_then(|after_a| {
                    if after_a.contains(b) {
                        Some(Ordering::Less)
                    } else {
                        Some(Ordering::Greater)
                    }
                })
                .unwrap_or_else(|| {
                    let after_b = orderings
                        .get(b)
                        .expect("A pair of pages have an ambiguous ordering!");
                    if after_b.contains(a) {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                })
        });

        if pages == old_pages {
            continue;
        }

        assert!(pages.len() & 1 == 1);
        total += pages[pages.len() / 2];
    }
    total
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day5.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{part1, part2};

    const TEXT: &str = indoc! {"
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
    "};

    #[test]
    fn part1_given_example() {
        assert_eq!(part1(TEXT), 143);
    }

    #[test]
    fn part2_given_example() {
        assert_eq!(part2(TEXT), 123);
    }
}
