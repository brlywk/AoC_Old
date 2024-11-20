use common::prelude::*;

fn main() {
    let input = Input::new("inputs/input.txt");
    let pt1 = part1(&input);
    println!("Part 1: {}", pt1);

    let pt2 = part2(&input);
    println!("Part 2: {}", pt2);
}

/*
 * A, X     Rock        1
 * B, Y     Paper       2
 * C, Z     Scissors    3
 *
 * Win                  6
 * Draw                 3
 * Loss                 0
 */
fn part1(input: &Input) -> i32 {
    input
        .lines
        .iter()
        .map(|line| {
            line.split_once(" ")
                .map(|(other, own)| {
                    let score = match own {
                        "X" => 1,
                        "Y" => 2,
                        "Z" => 3,
                        _ => 0,
                    };

                    let game_score = match (other, own) {
                        // draw
                        ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
                        // win
                        ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
                        // loss
                        ("A", "Z") | ("B", "X") | ("C", "Y") => 0,
                        _ => 0,
                    };

                    score + game_score
                })
                .unwrap()
        })
        .sum()
}

/*
 * Part 2:
 * X -> lose
 * Y -> draw
 * Z -> win
 *
 * A    Rock        1
 * B    Paper       2
 * C    Scissors    3
 */
fn part2(input: &Input) -> i32 {
    input
        .lines
        .iter()
        .map(|line| {
            line.split_once(" ")
                .map(|(other, own)| {
                    let outcome = match own {
                        "X" => 0,
                        "Y" => 3,
                        "Z" => 6,
                        _ => 0,
                    };

                    let to_play = match (own, other) {
                        // Lose
                        ("X", "A") => "C",
                        ("X", "B") => "A",
                        ("X", "C") => "B",
                        // Draw
                        ("Y", same) => same,
                        // Win
                        ("Z", "A") => "B",
                        ("Z", "B") => "C",
                        ("Z", "C") => "A",
                        _ => "",
                    };

                    let score = match to_play {
                        "A" => 1,
                        "B" => 2,
                        "C" => 3,
                        _ => 0,
                    };

                    outcome + score
                })
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::new("inputs/test.txt");
        let result = part1(&input);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_part2() {
        let input = Input::new("inputs/test.txt");
        let result = part2(&input);
        assert_eq!(result, 12);
    }
}
