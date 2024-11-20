use common::prelude::*;

fn main() {
    println!("AoC 2022 - Day 3");

    let input = Input::new("inputs/input.txt");

    let pt1 = part1(&input);
    println!("Part 1: {}", pt1);

    let pt2 = part2(&input);
    println!("Part 2: {}", pt2);
}

fn letter_to_number(c: char) -> u8 {
    if c.is_ascii_lowercase() {
        c as u8 - b'a' + 1
    } else {
        c as u8 - b'A' + 27
    }
}

fn intersect(str1: &str, str2: &str) -> char {
    str1.chars().filter(|c| str2.contains(*c)).next().unwrap()
}

fn intersect_more(str1: &str, str2: &str, str3: &str) -> char {
    str1.chars()
        .filter(|c| str2.contains(*c) && str3.contains(*c))
        .next()
        .unwrap()
}

fn part1(input: &Input) -> i32 {
    input
        .lines
        .iter()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            let same = intersect(a, b);

            letter_to_number(same) as i32
        })
        .sum()
}

fn part2(input: &Input) -> i32 {
    input
        .lines
        .chunks(3)
        .map(|chunk| {
            let [c1, c2, c3] = chunk else {
                panic!("invalid chunk count")
            };
            let c = intersect_more(c1, c2, c3);

            letter_to_number(c) as i32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_to_number() {
        assert_eq!(letter_to_number('a'), 1);
        assert_eq!(letter_to_number('z'), 26);
        assert_eq!(letter_to_number('A'), 27);
        assert_eq!(letter_to_number('Z'), 52);
    }

    #[test]
    fn test_intersect() {
        assert_eq!(intersect("abc", "ade"), 'a');
    }

    #[test]
    fn test_intersect_more() {
        assert_eq!(intersect_more("abc", "ade", "axy"), 'a');
    }

    #[test]
    fn test_pt1() {
        let input = Input::new("inputs/test.txt");
        let p1 = part1(&input);
        assert_eq!(p1, 157);
    }

    #[test]
    fn test_pte() {
        let input = Input::new("inputs/test.txt");
        let p2 = part2(&input);
        assert_eq!(p2, 70);
    }
}
