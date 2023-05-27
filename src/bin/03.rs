use std::vec;

pub fn part_one(input: &str) -> Option<u32> {
    // Get the most frequent bits
    let mut most = vec![0; 12];
    let mut line_length = 0;
    let mut first = true;
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if first {
                line_length += 1;
            }
            match c {
                '1' => most[i] += 1,
                '0' => most[i] -= 1,
                _ => unreachable!(),
            };
        }
        first = false;
    }

    // Turn "binary" vector of most frequent into a number
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for i in most {
        gamma <<= 1;
        epsilon <<= 1;

        if i > 0 {
            print!("1");
            gamma |= 1;
        } else {
            epsilon |= 1;
            print!("0");
        }

        line_length -= 1;
        if line_length == 0 {
            break;
        }
    }
    println!();
    println!("{}", gamma);
    println!("{}", epsilon);

    return Some(gamma * epsilon);
}

pub fn search1(lines: &mut Vec<&str>) -> u32 {
    let mut begin = 0;
    let mut end = lines.len();
    let mut index = 0;

    while begin + 1 != end {
        for line in &lines[begin..end] {
            println!("{}", line);
        }
        let middle = lines.get(begin + ((end - begin) / 2)).unwrap();
        println!("mid: {}", middle);

        let most_common = middle.chars().nth(index).unwrap();
        println!("most common: {}", most_common);
        if most_common == '1' {
            let first_most = lines[begin..end]
                .iter()
                .find(|x| x.chars().nth(index).unwrap() == most_common)
                .unwrap();
            println!("first most (moving beggining): {}", first_most);
            begin = lines.iter().position(|x| x == first_most).unwrap();
        } else {
            let last_most = lines[begin..end]
                .iter()
                .rfind(|x| x.chars().nth(index).unwrap() == most_common)
                .unwrap();
            println!("last most (moving ending): {}", last_most);
            end = lines.iter().position(|x| x == last_most).unwrap() + 1;
        }
        println!("{}, {}\n", begin, end);
        index += 1;
    }

    return u32::from_str_radix(lines.get(begin).unwrap(), 2).unwrap();
}

pub fn search0(lines: &Vec<&str>) -> u32 {
    let mut begin = 0;
    let mut end = lines.len();
    let mut index = 0;

    while begin + 1 != end {
        for line in &lines[begin..end] {
            println!("{}", line);
        }

        let middle = lines.get(begin + ((end - begin) / 2)).unwrap();
        println!("mid: {}", middle);

        let most_common = middle.chars().nth(index).unwrap();
        println!("most common: {}", most_common);
        if most_common == '0' {
            let first_most = lines[begin..end]
                .iter()
                .find(|x| x.chars().nth(index).unwrap() == '1')
                .unwrap();
            println!("first most (moving beggining): {}", first_most);
            begin = lines.iter().position(|x| x == first_most).unwrap();
        } else {
            let last_most = lines[begin..end]
                .iter()
                .rfind(|x| x.chars().nth(index).unwrap() == '0')
                .unwrap();
            println!("last most (moving ending): {}", last_most);
            end = 1 + lines.iter().position(|x| x == last_most).unwrap();
        }
        println!("{}, {}\n", begin, end);
        index += 1;
    }

    return u32::from_str_radix(lines.get(begin).unwrap(), 2).unwrap();
}

// pub fn part_two(input: &str) -> Option<u32> {
//     let mut lines: Vec<&str> = input.lines().collect();
//     lines.sort();
//     let most = search1(&mut lines);
//     let least = search0(&lines);
//     return Some(most * least);
// }

const WIDTH: usize = 5;

pub fn part_two(input: &str) -> Option<u32> {
    let nums = input
        .lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();

    let oxy = (0..WIDTH)
        .rev()
        .scan(nums.clone(), |oxy, i| {
            let one = oxy.iter().filter(|n| *n & 1 << i > 0).count() >= (oxy.len() + 1) / 2;
            let (_, oxyy): (Vec<_>, Vec<_>) = oxy
                .to_owned()
                .into_iter()
                .partition(|n| (*n & 1 << i > 0) != one);
            *oxy = oxyy;
            oxy.first().copied()
        })
        .last()
        .unwrap();

    let co2 = (0..WIDTH)
        .rev()
        .scan(nums, |co2, i| {
            let one = co2.iter().filter(|n| *n & 1 << i > 0).count() >= (co2.len() + 1) / 2;
            let (_, co2x): (Vec<_>, Vec<_>) = co2
                .to_owned()
                .into_iter()
                .partition(|n| (*n & 1 << i > 0) == one);
            *co2 = co2x;
            co2.first().copied()
        })
        .last()
        .unwrap();

    Some(oxy * co2)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(198));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(230));
    }
}
