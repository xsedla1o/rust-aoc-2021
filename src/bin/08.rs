use std::{char, vec};

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .into_iter()
            .flat_map(|line| {
                line.split_once('|')
                    .unwrap()
                    .1
                    .split(' ')
                    .map(|x| match x.len() {
                        // We only count the numbers which have a unique number of segments
                        2 | 4 | 3 | 7 => 1,
                        _ => 0,
                    })
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .into_iter()
            .map(|line| {
                let (all_nums, displayed) = line.split_once('|').unwrap();
                decode_display(a_little_deduction(all_nums), displayed)
            })
            .sum(),
    )
}

fn decode_display(segment_translate: Vec<u32>, display_nums: &str) -> u32 {
    let digits = display_nums
        .split(' ')
        .map(|digit| decode_digit(&segment_translate, digit));

    let mut multiplier: u32 = 1;
    digits
        .rev()
        .map(|d| {
            let mul_d = d * multiplier;
            multiplier *= 10;
            mul_d
        })
        .sum()
}

fn decode_digit(segment_translate: &[u32], digit: &str) -> u32 {
    match digit.len() {
        2 => 1,
        4 => 4,
        3 => 7,
        7 => 8,
        // 2, 3, 5
        5 => {
            let mut used_segments = vec![false; 7];
            digit
                .chars()
                .for_each(|x| used_segments[segment_translate[seg_ord(x)] as usize] = true);
            if !used_segments[2] {
                5
            } else if !used_segments[4] {
                3
            } else {
                2
            }
        }
        // 0, 6, 9
        6 => {
            let mut used_segments = vec![false; 7];
            digit
                .chars()
                .for_each(|x| used_segments[segment_translate[seg_ord(x)] as usize] = true);
            if !used_segments[2] {
                6
            } else if !used_segments[3] {
                0
            } else {
                9
            }
        }
        _ => 0,
    }
}

/**
 * We need to determine a translation of segments.
 * By looking at how many times each segment is used, we can determine segments b(6x), e(4x), f(9x)
 * Knowing f, we can determine c as the second of '1's segments
 * a can be determined as the difference between 7 (acf) and 1 (cf)
 * similarily, difference between 4 (bcdf) and 1 (cf) gives bd, but we know b already, so the other must be d
 * Only one remains - g.
 */
fn a_little_deduction(all_nums: &str) -> Vec<u32> {
    let mut segment_translate = vec![666; 7];
    let mut segment_counts: Vec<u32> = vec![0; 7];
    let mut digit_segments: Vec<&str> = vec![""; 10];

    for num in all_nums.split(' ') {
        // Save distinct digits
        match num.len() {
            2 => digit_segments[1] = num,
            4 => digit_segments[4] = num,
            3 => digit_segments[7] = num,
            _ => (),
        };

        // Save segment counts
        for char in num.chars() {
            segment_counts[seg_ord(char)] += 1;
        }
    }

    for (i, cnt) in segment_counts.iter().enumerate() {
        match cnt {
            4 => segment_translate[i] = 4,
            6 => segment_translate[i] = 1,
            9 => {
                segment_translate[i] = 5;
                digit_segments[1].chars().for_each(|char| {
                    let ord = seg_ord(char);
                    if ord != i {
                        segment_translate[ord] = 2;
                    }
                });
            }
            _ => (),
        }
    }

    for char in digit_segments[7].chars() {
        if !digit_segments[1].contains(char) {
            segment_translate[seg_ord(char)] = 0;
        }
    }

    for char in digit_segments[4].chars() {
        if !digit_segments[1].contains(char) && segment_translate[seg_ord(char)] == 666 {
            segment_translate[seg_ord(char)] = 3;
        }
    }

    let g_index = segment_translate.iter().position(|x| *x == 666).unwrap();
    segment_translate[g_index] = 6;
    // for translate in segment_translate.iter() {
    //     print!("{},", translate);
    // }
    // println!();

    segment_translate
}

fn seg_ord(char: char) -> usize {
    (char as u32 - 'a' as u32) as usize
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(61229));
    }
}
