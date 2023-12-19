type Output = usize;

fn hash(str: &str) -> usize {
    str.as_bytes()
        .iter()
        .fold(0, |acc, byte| (acc + *byte as usize) * 17 % 256)
}

fn part1(text: &str) -> Output {
    let mut sum = 0;
    for step in text.trim().split(',') {
        sum += hash(step);
    }

    sum
}

fn part2(text: &str) -> Output {
    let mut boxes = vec![vec![]; 256];
    for step in text.trim().split(',') {
        if step.ends_with('-') {
            let label = &step[..step.len() - 1];
            let box_i = hash(label);
            if let Some(index) = boxes[box_i]
                .iter()
                .enumerate()
                .find_map(|(i, (_, search_label))| (*search_label == label).then_some(i))
            {
                boxes[box_i].remove(index);
            }
        } else {
            let (label, focal_length) = step.split_once('=').unwrap();
            let focal_length = focal_length.parse::<usize>().unwrap();
            let box_i = hash(label);

            if let Some(index) = boxes[box_i]
                .iter()
                .enumerate()
                .find_map(|(i, (_, search_label))| (*search_label == label).then_some(i))
            {
                boxes[box_i][index] = (focal_length, label);
            } else {
                boxes[box_i].push((focal_length, label));
            }
        }
    }

    // for (i, items) in boxes.iter().enumerate() {
    //     if items.len() > 0 {
    //         println!("{i}: {:?}", items);
    //     }
    // }

    let mut sum = 0;
    for (mut box_i, items) in boxes.iter().enumerate() {
        box_i += 1;
        for (mut lens_slot, (focal_length, _label)) in items.iter().enumerate() {
            lens_slot += 1;
            // println!("{_label} {box_i} * {lens_slot} * {focal_length}");
            sum += box_i * lens_slot * focal_length;
        }
    }

    sum
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day15.txt")?;

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
