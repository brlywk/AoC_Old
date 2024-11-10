use common::prelude::*;

fn main() {
    println!("AoC 2022 - Day 01");

    let input = Input::new("01/inputs/pt1_input.txt");

    let pt1 = part1(&input).expect("part 1 failed");
    println!("part 1: {pt1}");

    let pt2 = part2(&input);
    println!("part 2: {pt2}");
}

fn group(input: &Input) -> Vec<usize> {
    input
        .lines
        .split(|line| line.is_empty())
        .map(|group| {
            group
                .iter()
                .map(|num_str| {
                    num_str
                        .parse::<usize>()
                        .expect(&format!("unable to parse {}", num_str))
                })
                .sum()
        })
        .collect()
}

fn part1(input: &Input) -> Option<usize> {
    group(input).iter().max().copied()
}

fn part2(input: &Input) -> usize {
    let mut groups = group(&input);
    groups.sort_by(|a, b| b.cmp(a));
    groups.iter().take(3).sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn pt1_test() {
        let test_input = Input::new("inputs/pt1_test.txt");

        assert_eq!(Some(24000), part1(&test_input));
    }

    #[test]
    fn pt2_test() {
        let test_input = Input::new("inputs/pt1_test.txt");

        assert_eq!(45000, part2(&test_input));
    }
}
