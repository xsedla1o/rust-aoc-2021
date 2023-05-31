use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut crabs = HashMap::new();
    input
        .split_once('\n')
        .unwrap()
        .0
        .split(',')
        .map(|n| n.parse().unwrap())
        .for_each(|p: u32| *crabs.entry(p).or_insert(0) += 1);

    // crabs.iter().for_each(|x| println!("{}: {}", x.0, x.1));

    (*crabs.keys().min().unwrap()..=*crabs.keys().max().unwrap())
        .map(|target| {
            crabs.iter().fold(0, |sum, (&pos, &cnt)| {
                sum + target.abs_diff(pos) * cnt as u32
            })
        })
        .min()
}

fn fuel_cost(distance: u32) -> u32 {
    // (0..=distance).sum()
    ((1 + distance) as f64 / 2.0 * distance as f64) as u32
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut crabs = HashMap::new();
    input
        .split_once('\n')
        .unwrap()
        .0
        .split(',')
        .map(|n| n.parse().unwrap())
        .for_each(|p: u32| *crabs.entry(p).or_insert(0) += 1);

    // crabs.iter().for_each(|x| println!("{}: {}", x.0, x.1));

    (*crabs.keys().min().unwrap()..=*crabs.keys().max().unwrap())
        .map(|target| {
            crabs.iter().fold(0, |sum, (&pos, &cnt)| {
                sum + fuel_cost(target.abs_diff(pos)) * cnt as u32
            })
        })
        .min()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(37));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(168));
    }
}
