use std::{
    cmp::{max, min},
    collections::HashMap,
};

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: u32,
    y: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut floor_map: HashMap<Point, u32> = HashMap::new();

    input.lines().for_each(|line| {
        let points: Vec<Point> = line
            .split(" -> ")
            .map(|s_point| {
                let (x, y) = s_point.split_once(',').unwrap();
                Point {
                    x: str::parse(x).unwrap(),
                    y: str::parse(y).unwrap(),
                }
            })
            .collect();
        match &points[..] {
            [a, b, ..] if a.x == b.x => {
                for i in min(a.y, b.y)..=max(a.y, b.y) {
                    let p = Point { x: a.x, y: i };
                    inc(&mut floor_map, p);
                }
            }
            [a, b, ..] if a.y == b.y => {
                for i in min(a.x, b.x)..=max(a.x, b.x) {
                    let p = Point { x: i, y: a.y };
                    inc(&mut floor_map, p);
                }
            }
            [_, _, ..] => (),
            _ => unreachable!(),
        }
    });

    Some(floor_map.iter().filter(|(_p, val)| **val >= 2).count() as u32)
}

fn inc(hmap: &mut HashMap<Point, u32>, key: Point) {
    // println!("{:?}", key);
    match hmap.get(&key) {
        Some(&value) => hmap.insert(key, value + 1),
        None => hmap.insert(key, 1),
    };
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut floor_map: HashMap<Point, u32> = HashMap::new();

    input.lines().for_each(|line| {
        let points: Vec<Point> = line
            .split(" -> ")
            .map(|s_point| {
                let (x, y) = s_point.split_once(',').unwrap();
                Point {
                    x: str::parse(x).unwrap(),
                    y: str::parse(y).unwrap(),
                }
            })
            .collect();
        match &points[..] {
            [a, b, ..] if a.x == b.x => {
                for i in min(a.y, b.y)..=max(a.y, b.y) {
                    let p = Point { x: a.x, y: i };
                    inc(&mut floor_map, p);
                }
            }
            [a, b, ..] if a.y == b.y => {
                for i in min(a.x, b.x)..=max(a.x, b.x) {
                    let p = Point { x: i, y: a.y };
                    inc(&mut floor_map, p);
                }
            }
            [a, b, ..] => {
                let x_step = (b.x as i32 - a.x as i32).signum();
                let y_step = (b.y as i32 - a.y as i32).signum();
                let (mut x, mut y) = (a.x as i32, a.y as i32);
                loop {
                    let p = Point {
                        x: x as u32,
                        y: y as u32,
                    };
                    if p == *b {
                        inc(&mut floor_map, p);
                        break;
                    }
                    inc(&mut floor_map, p);

                    x += x_step;
                    y += y_step;
                }
            }
            _ => unreachable!(),
        }
    });

    Some(floor_map.iter().filter(|(_p, val)| **val >= 2).count() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(12));
    }
}
