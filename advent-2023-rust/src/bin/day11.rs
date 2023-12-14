type Output = usize;

fn part1(text: &str) -> Output {
    let mut lines = text.lines().peekable();

    let mut in_column = vec![false; lines.peek().unwrap().len()];
    let mut in_row = false;
    let mut row_index = 0 as usize;
    let mut galaxies = vec![];
    for line in lines {
        for (i, char) in line.chars().enumerate() {
            if char == '#' {
                in_row = true;
                in_column[i] = true;
                galaxies.push((row_index, i));
            }
        }
        row_index += if in_row { 1 } else { 2 };
        in_row = false;
    }

    let mut offset = vec![];
    let mut index = 0;
    for column in in_column {
        offset.push(index);
        if !column {
            index += 1;
        }
    }

    galaxies = galaxies
        .iter()
        .map(|(row, col)| (*row, col + offset[*col]))
        .collect();

    return galaxies
        .iter()
        .enumerate()
        .map(|(i, (col1, row1))| {
            galaxies
                .iter()
                .skip(i + 1)
                .map(|(col2, row2)| col1.abs_diff(*col2) + row1.abs_diff(*row2))
                .sum::<usize>()
        })
        .sum();
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day11.txt")?;

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
