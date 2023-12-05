use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

struct MatrixIndex {
    row: usize,
    column: usize,
}

fn gear_locations(top: &str, middle: &str, bottom: &str, slice: Range<usize>) -> Vec<MatrixIndex> {
    let Range { start, end } = slice;
    let mut result = vec![];
    if start > 0 && middle.chars().nth(start - 1).unwrap() == '*' {
        result.push(MatrixIndex {
            row: 1,
            column: start - 1,
        })
    } else if let Some(symbol) = middle.chars().nth(end) {
        if symbol == '*' {
            result.push(MatrixIndex {
                row: 1,
                column: end,
            })
        }
    }
    let min = if start == 0 { 0 } else { start - 1 };
    let min_top = std::cmp::min(min, top.len() - 1);
    let min_bottom = std::cmp::min(min, bottom.len() - 1);
    let max_top = std::cmp::min(end + 1, top.len());
    let max_bottom = std::cmp::min(end + 1, bottom.len());

    for (i, char) in top[min_top..max_top].chars().enumerate() {
        if char == '*' {
            result.push(MatrixIndex {
                row: 0,
                column: min_top + i,
            })
        }
    }
    for (i, char) in bottom[min_bottom..max_bottom].chars().enumerate() {
        if char == '*' {
            result.push(MatrixIndex {
                row: 2,
                column: min_bottom + i,
            })
        }
    }
    return result;
}

fn main() -> io::Result<()> {
    let f = File::open("./assets/day3.txt")?;
    let mut lines = io::BufReader::new(f).lines().chain([Ok(".".into())]);
    let mut top = ".".to_string();
    let mut middle = lines.next().unwrap().unwrap();

    let mut gears = HashMap::new();

    for (row_index, bottom) in lines.enumerate() {
        let bottom = bottom.unwrap();
        let mut digits = None;
        let mut start_index = 0;

        for (i, char) in middle.chars().chain(['.']).enumerate() {
            match (char.is_ascii_digit(), &mut digits) {
                (true, None) => {
                    start_index = i;
                    digits = Some(char.to_string())
                }
                (true, Some(ref mut str)) => str.push(char),

                (false, None) => (),
                (false, Some(ref str)) => {
                    let number: u32 = str.parse().unwrap();
                    digits = None;

                    for MatrixIndex { row, column } in
                        gear_locations(&top, &middle, &bottom, start_index..i)
                    {
                        gears
                            .entry((row_index + row - 1, column))
                            .or_insert(vec![])
                            .push(number)
                    }
                }
            }
        }

        (top, middle) = (middle, bottom);
    }

    let mut sum = 0;
    for numbers in gears.values() {
        if numbers.len() == 2 {
            sum += numbers[0] * numbers[1];
        }
    }

    println!("sum = {}", sum);
    Ok(())
}
