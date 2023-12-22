advent_of_code::solution!(9);

fn parse_input(input: &str, reverse: bool) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            let mut new_line: Vec<i64> = line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            if reverse {
                new_line.reverse();
            }
            new_line
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    let sequences = parse_input(input, false);

    let answer = compute_answer(sequences);

    Some(answer)
}

pub fn part_two(input: &str) -> Option<i64> {
    let sequences = parse_input(input, true);

    let answer = compute_answer(sequences);

    Some(answer)
}

fn compute_answer(sequences: Vec<Vec<i64>>) -> i64 {
    sequences
        .into_iter()
        .map(|seq| {
            let mut triangle = vec![seq];
            while !triangle.last().unwrap().iter().all(|&n| n == 0) {
                let last_line = triangle.last().unwrap();
                let next_line = (0..(last_line.len() - 1)).map(|i| last_line[i + 1] - last_line[i]);
                triangle.push(next_line.collect());
            }
            triangle
                .iter()
                .rev()
                .fold(0, |acc, line| acc + line.last().unwrap())
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
