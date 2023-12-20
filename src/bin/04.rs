use std::iter::Map;
use std::str::Lines;
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let win_counts = get_win_counts(input);
    let answer = win_counts
        .map(|n| match n {
            0 => 0,
            n => 1u32 << (n - 1),
        })
        .sum();

    Some(answer)
}

fn get_win_counts(input: &str) -> Map<Lines, fn(&str) -> usize> {
    input.lines().map(|line| {
        let all_numbers = line.split_once(":").unwrap().1;
        let (winning, mine) = all_numbers.split_once("|").unwrap();
        let winning = winning
            .split_whitespace()
            .map(|str| str.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let mine = mine
            .split_whitespace()
            .map(|str| str.parse::<u32>().unwrap());
        mine.filter(|n| winning.contains(n)).count()
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut n_copies = vec![1; input.lines().count()];

    let win_counts = get_win_counts(input);

    for (card, wins) in win_counts.enumerate() {
        for card_won in (card + 1)..=(card + wins) {
            n_copies[card_won] += n_copies[card];
        }
    }

    Some(n_copies.iter().sum())
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
