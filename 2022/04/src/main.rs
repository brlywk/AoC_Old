use std::ops::RangeInclusive;

use common::prelude::*;

fn main() {
    println!("AoC 2022 - Day 4");

    let input = Input::new("inputs/input.txt");

    let pt1 = do_the_thing(&input, within);
    println!("Part 1: {}", pt1);

    let pt2 = do_the_thing(&input, overlap);
    println!("Part 2: {}", pt2);
}

fn str_to_range(str: &str) -> RangeInclusive<i32> {
    let (s, e) = str.split_once("-").unwrap();

    let start = s.parse::<i32>().unwrap();
    let end = e.parse::<i32>().unwrap();

    RangeInclusive::new(start, end)
}

fn within(r1: RangeInclusive<i32>, r2: RangeInclusive<i32>) -> bool {
    r1.start() >= r2.start() && r1.end() <= r2.end()
        || r2.start() >= r1.start() && r2.end() <= r1.end()
}

fn overlap(r1: RangeInclusive<i32>, r2: RangeInclusive<i32>) -> bool {
    r1.start() <= r2.end() && r1.end() >= r2.start()
}

fn do_the_thing(
    input: &Input,
    comparer: fn(RangeInclusive<i32>, RangeInclusive<i32>) -> bool,
) -> usize {
    input
        .lines
        .iter()
        .filter(|line| {
            line.split_once(",")
                .map(|(s1, s2)| {
                    let (r1, r2) = (str_to_range(s1), str_to_range(s2));

                    comparer(r1, r2)
                })
                .unwrap()
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_range() {
        assert_eq!(str_to_range("1-3"), 1..=3);
        assert_eq!(str_to_range("2-4"), 2..=4);
        assert_eq!(str_to_range("6-8"), 6..=8);
    }

    #[test]
    fn test_range_within() {
        assert_eq!(within(1..=10, 1..=5), true);
        assert_eq!(within(1..=5, 1..=10), true);
    }

    #[test]
    fn test_part1() {
        let input = Input::new("inputs/test.txt");
        let p1 = do_the_thing(&input, within);
        assert_eq!(p1, 2);
    }

    #[test]
    fn test_overlap() {
        assert_eq!(overlap(5..=7, 7..=9), true);
        assert_eq!(overlap(2..=4, 6..=8), false);
    }

    #[test]
    fn test_part2() {
        let input = Input::new("inputs/test.txt");
        let p2 = do_the_thing(&input, overlap);
        assert_eq!(p2, 4);
    }
}
