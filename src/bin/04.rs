use std::cmp::min;
use std::str::Lines;

const SIZE: usize = 5;

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let nums_in: Vec<u32> = get_input_nums(&mut lines);
    let mut boards: Vec<Vec<u32>> = get_boards(lines);

    boards.sort_by_key(|x| nums_to_win(x, &nums_in));

    Some(board_score(&boards[0], &nums_in))
}

fn get_input_nums(lines: &mut Lines) -> Vec<u32> {
    lines
        .next()
        .as_mut()
        .unwrap()
        .split(',')
        .map(|n| str::parse(n).unwrap())
        .collect()
}

fn get_boards(lines: Lines) -> Vec<Vec<u32>> {
    let nums: Vec<u32> = lines
        .flat_map(|x| x.split(' '))
        .map(str::parse)
        .filter(|res| !res.is_err())
        .map(|res| res.unwrap())
        .collect();

    nums.chunks(SIZE * SIZE)
        .map(|chunk| chunk.to_owned())
        .collect()
}

fn board_score(board: &[u32], nums_in: &[u32]) -> u32 {
    let max_i = nums_to_win(board, nums_in);
    let unchecked_sum: u32 = board
        .iter()
        .filter(|x| !nums_in[0..=max_i].contains(x))
        .sum();

    unchecked_sum * nums_in[max_i]
}

fn win_by<'a, B: Iterator<Item = Row>, Row: IntoIterator<Item = &'a u32>>(
    board_view: B,
    nums_in: &'a [u32],
) -> usize {
    board_view // Either the rows or cols
        .map(|row: Row| {
            row.into_iter()
                .map(|n| nums_in.iter().position(|x| *x == *n).unwrap())
                .max()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn nums_to_win(board: &[u32], nums: &[u32]) -> usize {
    let rows = board.chunks(SIZE);
    let cols = (0..SIZE).map(|i| board.iter().skip(i).step_by(SIZE).take(SIZE));

    min(win_by(rows, nums), win_by(cols, nums))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let nums_in: Vec<u32> = get_input_nums(&mut lines);
    let mut boards: Vec<Vec<u32>> = get_boards(lines);

    boards.sort_by_key(|x| nums_to_win(x, &nums_in));

    Some(board_score(&boards[0], &nums_in))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(4512));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(1924));
    }
}
