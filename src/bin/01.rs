use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut first = true;
    let mut curr_depth: u32 = 0;
    let mut new_depth: u32;
    let mut decrease_cnt: u32 = 0;
    for line in lines {
        if first {
            curr_depth = line.parse::<u32>().unwrap();
            first = false;
        } else {
            // print!("{}\n", curr_depth);
            new_depth = line.parse::<u32>().unwrap();
            if new_depth > curr_depth {
                decrease_cnt += 1;
            }
            curr_depth = new_depth;
        }
    }
    Some(decrease_cnt)
}

pub fn print_deque(deque: &VecDeque<u32>) {
    for item in deque.iter() {
        print!("{}, ", item);
    }
    print!("\n");
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut deque: VecDeque<u32> = VecDeque::with_capacity(4);
    let mut depth: u32;
    let mut decrease_cnt: u32 = 0;
    for line in lines {
        if deque.len() < 3 {
            depth = line.parse::<u32>().unwrap();
            deque.push_back(depth);
            // println!("{}", depth);
        } else {
            depth = line.parse::<u32>().unwrap();
            deque.push_back(depth);
            // print_deque(&deque);

            let mut curr_sum = 0;
            for i in 0..3 {
                curr_sum += deque.get(i).unwrap();
                // println!("- {}", deque.get(i).unwrap());
            }
            // println!("->{}", curr_sum);

            let mut new_sum = 0;
            for i in 1..4 {
                new_sum += deque.get(i).unwrap();
                // println!("- {}", deque.get(i).unwrap());
            }
            // println!("->{}", new_sum);

            if new_sum > curr_sum {
                decrease_cnt += 1;
                // println!("Increase")
            }

            deque.pop_front();
        }
        // println!();
    }
    Some(decrease_cnt)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(5));
    }
}
