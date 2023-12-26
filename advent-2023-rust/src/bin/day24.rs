type Output = usize;

fn part1(text: &str) -> Output {
    let mut tmp = vec![];
    for line in text.lines() {
        let (x, line) = line.split_once(", ").unwrap();
        let (y, line) = line.split_once(", ").unwrap();
        let (z, line) = line.split_once(" @ ").unwrap();
        let (dx, line) = line.split_once(", ").unwrap();
        let (dy, dz) = line.split_once(", ").unwrap();
        let [x, y, _z] = [x, y, z].map(|s| s.parse::<usize>().unwrap());
        let [dx, dy, _dz] = [dx.trim(), dy.trim(), dz.trim()].map(|s| s.parse::<i32>().unwrap());

        // Thank you for making all diffs non-zero
        assert!(dx != 0);
        assert!(dy != 0);
        assert!(_dz != 0);
        tmp.push((x, y, dx, dy, dy as f64 / dx as f64));
    }

    let mut inside = 0;
    let mut number = 0;
    for (ai, a) in tmp.iter().enumerate() {
        for (bi, b) in tmp.iter().enumerate().skip(ai + 1) {
            number += 1;
            if ai == bi {
                continue;
            }

            let (x1, y1, dx1, _dy1, m1) = *a;
            let (x2, y2, dx2, _dy2, m2) = *b;
            let c1 = y1 as f64 - m1 * x1 as f64;
            let c2 = y2 as f64 - m2 * x2 as f64;

            if m1 == m2 {
                if c1 == c2 {
                    panic!(
                        "Paths lie on each other: {:?}, {:?}",
                        (x1, y1, m1, c1),
                        (x2, y2, m2, c2)
                    );
                } else {
                    // println!("{:?}", (a, b, "parallel"));
                    continue;
                }
            }
            // y1 = m1*x1 + c1
            // y2 = m2*x2 + c2
            // c1 = y1 - m1*x1, etc.
            // When (x1, y1) = (x2, y2)
            // m1*x + c1 = m2*x + c2
            // x = (c2 - c1) / (m1 - m2)
            // y = ...

            const BOUNDS: (f64, f64) = (2e14, 4e14);

            let x_intersect = (c2 - c1) / (m1 - m2);
            let y_intersect = m1 * x_intersect + c1;

            let (_message, diff) = match (
                (x1 as f64).partial_cmp(&x_intersect) == 0.partial_cmp(&dx1),
                (x2 as f64).partial_cmp(&x_intersect) == 0.partial_cmp(&dx2),
            ) {
                (false, false) => ("both in past", 0),
                (false, true) => ("first in past", 0),
                (true, false) => ("second in past", 0),
                (true, true)
                    if x_intersect < BOUNDS.0
                        || x_intersect > BOUNDS.1
                        || y_intersect < BOUNDS.0
                        || y_intersect > BOUNDS.1 =>
                {
                    ("outside the test area", 0)
                }
                (true, true) => ("inside the test area", 1),
            };
            inside += diff
            // println!("{:?}", (x1, y1, x2, y2, x_intersect, y_intersect, _message));
        }
    }

    let len = tmp.len();
    println!("{number}; {len} => {}", len * (len - 1) / 2);
    inside
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day24.txt")?;

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
