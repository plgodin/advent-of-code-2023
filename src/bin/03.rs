use regex::Regex;
advent_of_code::solution!(3);

struct Position {
    x: i32,
    y: i32,
}

struct Number {
    value: u32,
    pos: Position,
}

impl Number {
    fn is_adjacent(&self, pos: &Position) -> bool {
        let width = (self.value.ilog10() + 1) as i32;
        pos.x >= self.pos.x - 1
            && pos.x <= self.pos.x + width
            && pos.y >= self.pos.y - 1
            && pos.y <= self.pos.y + 1
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (numbers, symbols) = parse_input(input, false)?;

    let answer = numbers
        .iter()
        .filter(|num| symbols.iter().any(|s| num.is_adjacent(s)))
        .map(|num| num.value)
        .sum();

    Some(answer)
}

fn parse_input(input: &str, stars_only: bool) -> Option<(Vec<Number>, Vec<Position>)> {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    let number_regex = Regex::new("\\d+").ok()?;
    let symbol_regex = Regex::new("[^\\d.]").ok()?;

    for (y, line) in input.lines().enumerate() {
        // Find all numbers
        for m in number_regex.find_iter(line) {
            numbers.push(Number {
                value: m.as_str().parse().ok()?,
                pos: Position {
                    x: m.start() as i32,
                    y: y as i32,
                },
            });
        }
        // Find all symbols
        for m in symbol_regex.find_iter(line) {
            if m.as_str() == "*" || !stars_only {
                symbols.push(Position {
                    x: m.start() as i32,
                    y: y as i32,
                })
            }
        }
    }
    Some((numbers, symbols))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (numbers, stars) = parse_input(input, true)?;

    let gear_ratios = stars.iter().filter_map(|s| {
        let adjacent_numbers = numbers
            .iter()
            .filter(|&n| n.is_adjacent(s))
            .collect::<Vec<_>>();
        if adjacent_numbers.len() == 2 {
            Some(adjacent_numbers.iter().map(|n| n.value).product::<u32>())
        } else {
            None
        }
    });

    Some(gear_ratios.sum())
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
