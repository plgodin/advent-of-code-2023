use std::str::Chars;
advent_of_code::solution!(7);

fn score(hand: Chars, with_jokers: bool) -> u32 {
    // Map the Chars to a Vector of u32
    let hand = hand
        .map(|c| {
            if let Some(n) = c.to_digit(10) {
                return n;
            } else {
                match c {
                    'T' => 10,
                    'J' => {
                        if with_jokers {
                            1
                        } else {
                            11
                        }
                    }
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => {
                        panic!()
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    // Count the number of each card
    let mut card_counts = hand.iter().fold([0u32; 15], |mut state, &card| {
        state[card as usize] += 1;
        state
    });

    // Calculate the tiebreaker score for hands of the same type
    let tiebreaker = hand.iter().fold(0u32, |score, n| (score << 4) + n);

    // Find the most common card (not how many of it we have)
    let most_common_card = card_counts
        .iter()
        .enumerate()
        .fold((0, 0), |acc, (card, &count)| {
            if count > acc.1 && card > 1 {
                (card, count)
            } else {
                acc
            }
        })
        .0;

    // Increase the count of the most common card by the number of jokers we have
    card_counts[most_common_card] += card_counts[1];

    // Find our base score (the hand type)
    let base_score = match card_counts[2..].iter().max().unwrap() {
        5 => 60_000_000,
        4 => 50_000_000,
        3 => {
            if card_counts[2..].contains(&2) {
                40_000_000
            } else {
                30_000_000
            }
        }
        2 => {
            if card_counts[2..].iter().filter(|&n| *n == 2).count() == 2 {
                20_000_000
            } else {
                10_000_000
            }
        }
        1 => 0,
        0 => 60_000_000, // All jokers!
        _ => panic!(),
    };

    base_score + tiebreaker
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<(u32, u32)> = input
        .lines()
        .map(|line| {
            (
                score(line[0..=4].chars(), false),
                line[6..].parse().unwrap(),
            )
        })
        .collect();

    hands.sort_by_key(|&(score, _)| score);

    let answer = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i as u32 + 1) * hand.1);

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<(u32, u32)> = input
        .lines()
        .map(|line| (score(line[0..=4].chars(), true), line[6..].parse().unwrap()))
        .collect();

    hands.sort_by_key(|&(score, _)| score);

    let answer = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (i as u32 + 1) * hand.1);

    Some(answer)
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
