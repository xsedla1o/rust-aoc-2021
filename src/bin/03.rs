use std::{vec};

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

pub fn search1(lines: &mut Vec<&str>) -> u32{
    let mut begin = 0;
    let mut end = lines.len();
    let mut index = 0;

    while begin + 1 != end {
        for line in &lines[begin..end] {
            println!("{}", line);
        }
        // lines[begin..end].sort_by(|x, y| x.chars().nth(index).unwrap().cmp(&y.chars().nth(index).unwrap()));
        let middle = lines.get(begin + ((end - begin) / 2)).unwrap();
        println!("mid: {}", middle);

        let most_common = middle.chars().nth(index).unwrap();
        println!("most common: {}", most_common);
        if most_common == '1' {
            let first_most = lines[begin..end].iter()
                .find(|x| x.chars().nth(index).unwrap() == most_common)
                .unwrap();
            println!("first most (moving beggining): {}", first_most);
            begin = lines.iter().position(|x| x == first_most).unwrap();
        } else {
            let last_most = lines[begin..end].iter()
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

pub fn search0(lines: &Vec<&str>) -> u32{
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
            let first_most = lines[begin..end].iter()
                .find(|x| x.chars().nth(index).unwrap() == '1')
                .unwrap();
            println!("first most (moving beggining): {}", first_most);
            begin = lines.iter().position(|x| x == first_most).unwrap();
        } else {
            let last_most = lines[begin..end].iter()
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

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines: Vec<&str> = input.lines().collect();
    lines.sort();
    let most = search1(&mut lines);
    let least = search0(&lines);
    return Some(most * least);
}

// let mut most_common: Vec<u32> = Vec::new();
// let mut least_common: Vec<u32>  = Vec::new();
// let mut most_common_lines: Vec<&str> = input.lines().collect();
// let mut least_common_lines: Vec<&str> = input.lines().collect();
// let mut most_common_n;
// let mut least_common_n;
// let mut index = 0;
// while most_common_lines.len() != 1 && least_common_lines.len() != 1 {
//     most_common_n = 0;
//     least_common_n = 0;
//     for line in most_common_lines {
//         match line.chars().nth(index).unwrap() {
//             '1' => most_common_n += 1,
//             '0' => least_common_n -= 1,
//             _ => unreachable!(),
//         };
//     }

//     if most_common_n >= 0 {
//         most_common.push(1);
//         least_common.push(0);
//     } else {
//         most_common.push(1);
//         least_common.push(0);
//     }

//     most_common_lines = input.lines()
//         .filter(|x| x.chars().nth(index).unwrap().to_digit(10).unwrap()
//                             == most_common[index])
//         .collect();


//     least_common_lines = input.lines()
//         .filter(|x| x.chars().nth(index).unwrap().to_digit(10).unwrap()
//                             == least_common[index])
//         .collect();

//     for line in &most_common_lines {
//         println!("{}", line);
//     }
//     println!();

//     for line in &least_common_lines {
//         println!("{}", line);
//     }
//     println!();
//     index += 1;
//     if index == 5 {
//         break;
//     }    
// }

// return Some(0);

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
