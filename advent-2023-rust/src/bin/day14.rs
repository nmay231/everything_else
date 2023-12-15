type Output = usize;

fn part1(text: &str) -> Output {
    let mut lines = text.lines().peekable();

    let mut load = text.lines().count();
    let row_length = lines.peek().unwrap().len();
    let mut loads = vec![load; row_length];

    let mut total_load = 0;
    for line in lines {
        load -= 1;
        for (col, char) in line.chars().enumerate() {
            match char {
                '#' => loads[col] = load,
                'O' => {
                    total_load += loads[col];
                    loads[col] -= 1;
                }
                '.' => (),
                _ => panic!("Unexpected char in puzzle input: '{char}'"),
            }
        }
    }

    total_load
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day14.txt")?;

    println!("part 1 result = {:?}", part1(&text));
    println!("part 2 result = {:?}", part2(&text));

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn sample() {
        assert!(true);
    }
}
