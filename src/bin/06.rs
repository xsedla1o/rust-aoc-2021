pub fn part_one(input: &str) -> Option<u32> {
    let mut curr_fish: Vec<u32> = vec![0; 9];
    let mut next_fish: Vec<u32> = vec![0; 9];
    input
        .lines()
        .flat_map(|line| line.split(','))
        .map(|s| s.parse().unwrap())
        .for_each(|x: usize| {
            curr_fish[x] += 1;
        });

    for _ in 0..80 {
        // curr_fish.iter().for_each(|x| print!("{},", x));
        // println!();

        for (i, cnt) in curr_fish.iter().enumerate() {
            match i {
                0 => {
                    next_fish[6] += cnt;
                    next_fish[8] += cnt;
                }
                n => next_fish[n - 1] += cnt,
            }
        }

        curr_fish = next_fish;
        next_fish = vec![0; 9];
    }

    Some(curr_fish.iter().sum())
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut curr_fish: Vec<u128> = vec![0; 9];
    let mut next_fish: Vec<u128> = vec![0; 9];
    input
        .lines()
        .flat_map(|line| line.split(','))
        .map(|s| s.parse().unwrap())
        .for_each(|x: usize| {
            curr_fish[x] += 1;
        });

    for _ in 0..256 {
        // curr_fish.iter().for_each(|x| print!("{},", x));
        // println!();

        for (i, cnt) in curr_fish.iter().enumerate() {
            match i {
                0 => {
                    next_fish[6] += cnt;
                    next_fish[8] += cnt;
                }
                n => next_fish[n - 1] += cnt,
            }
        }

        curr_fish = next_fish;
        next_fish = vec![0; 9];
    }

    Some(curr_fish.iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(5934));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(26984457539));
    }
}
