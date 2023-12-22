use std::collections::HashMap;

use advent_2023_rust::{Direc, UsizePoint};

type Output = usize;

// I feel pretty proud of myself, not gonna lie. I wrote this all, fixed
// three bugs on the example case in 5 minutes, got the right answer, then
// got the right answer on the final one with no changes. Pretty hyped right now.
fn part1(text: &str) -> Output {
    let grid_size = &UsizePoint(text.lines().count(), text.lines().next().unwrap().len());
    let grid = text.trim().replace('\n', "").chars().collect::<Vec<_>>();

    // HashMap<(point, current_direction), min_heat_loss>
    let mut min_heat_loss = HashMap::new();
    // cost = distance_to_goal + accumulated_heat_loss
    // Vec<(cost, point, next_direction, current_heat_loss)>
    let mut paths = Vec::new();

    let target_point = UsizePoint(grid_size.0 - 1, grid_size.1 - 1);
    const DISTANCE_COST_MULTIPLIER: usize = 1;
    let start_cost = DISTANCE_COST_MULTIPLIER * (target_point.0 + target_point.1);
    paths.extend([
        (start_cost, UsizePoint(0, 0), Direc::East, 0),
        (start_cost, UsizePoint(0, 0), Direc::South, 0),
    ]);
    let mut best_winning_cost = None;

    while let Some((mut cost, mut point, direc, mut heat_loss)) = paths.pop() {
        let left = direc.rotate(1);
        let right = direc.rotate(-1);
        for _ in 0..3 {
            point = match point.next_point(&direc, grid_size) {
                Some(p) => p,
                None => break, // Grid edge
            };
            let cell_heat_loss = grid[point.as_index(grid_size)].to_digit(10).unwrap();
            heat_loss += cell_heat_loss;

            match direc {
                Direc::East | Direc::South => cost -= DISTANCE_COST_MULTIPLIER, // Go closer to the goal
                Direc::West | Direc::North => cost += DISTANCE_COST_MULTIPLIER,
            };
            cost += cell_heat_loss as usize;

            if let Some(best_heat_loss) = min_heat_loss.get(&(point, direc)) {
                if *best_heat_loss <= heat_loss {
                    // TODO: If I need shortest path as well as lowest heat_loss in part2, I need to change this.
                    continue;
                }
            }

            if point == target_point
                && (best_winning_cost.is_none() || cost < best_winning_cost.unwrap())
            {
                if best_winning_cost.is_none() {
                    paths = paths
                        .into_iter()
                        .skip_while(|(existing_cost, point, _, _)| {
                            let remaining_distance =
                                target_point.0 + target_point.1 - point.0 - point.1;
                            *existing_cost >= cost + DISTANCE_COST_MULTIPLIER * remaining_distance
                        })
                        .collect();
                }
                assert_eq!(cost, heat_loss as usize, "cost is different than heat_loss at target_point. Did not subtract distance at some point");
                best_winning_cost = Some(cost);
            }

            min_heat_loss.insert((point, direc), heat_loss);

            // Keep `paths` sorted by cost so the least costly option is popped off first
            let index = paths
                .iter()
                .enumerate()
                .rev()
                .find_map(|(i, (existing_cost, _, _, _))| (*existing_cost > cost).then_some(i + 1))
                .unwrap_or(0);
            paths
                .splice(
                    index..index,
                    [
                        (cost, point.to_owned(), left.to_owned(), heat_loss),
                        (cost, point.to_owned(), right.to_owned(), heat_loss),
                    ],
                )
                .count(); // Consume the iterator (I don't know if this is needed to add the elements, tbh)
        }
    }

    return best_winning_cost.expect("Didn't find a winning cost I guess");
}

fn part2(_text: &str) -> Output {
    0
}

fn main() -> std::io::Result<()> {
    let text = std::fs::read_to_string("./assets/day17.txt")?;

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
