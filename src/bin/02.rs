pub fn part_one(input: &str) -> Option<u32> {
    let mut x: u32 = 0;
    let mut y: u32 = 0;
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let direction = split.next().unwrap();
        let distance = split.next().unwrap().parse::<u32>().unwrap();
        match direction {
            "forward" => x += distance,
            "down" => y += distance,
            "up" => y -= distance,
            _ => panic!("Unknown direction"),
        }
    }
    Some(x * y)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut aim: i32 = 0;
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let direction = split.next().unwrap();
        let distance = split.next().unwrap().parse::<i32>().unwrap();
        match direction {
            "forward" => {
                x += distance;
                y += distance * aim
            }
            "down" => aim += distance,
            "up" => aim -= distance,
            _ => panic!("Unknown direction"),
        }
    }
    Some((x * y).try_into().unwrap())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(150));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(900));
    }
}
