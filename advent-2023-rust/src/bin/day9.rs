type Result = i32;

fn part1(text: &str) -> Result {
    let mut sum = 0;
    for line in text.lines() {
        let mut numbers = line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut last_numbers = vec![*numbers.last().unwrap()];
        loop {
            numbers = numbers.windows(2).map(|pair| pair[1] - pair[0]).collect();
            let last = *numbers.last().unwrap();
            if last == 0 && numbers.iter().nth_back(1).unwrap() == &0 {
                break;
            }
            last_numbers.push(last);
        }
        sum += last_numbers.iter().sum::<i32>();
    }
    return sum;
}

fn part2(text: &str) -> Result {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day9.txt")?;

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
