advent_of_code::solution!(2);

use regex::Regex;
use std::cmp::max;

// We could probably do this without regexes, but I'm sure we'll have to use them at some point anyway
pub fn part_one(input: &str) -> Option<u32> {
    let answer = input
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            let regex = Regex::new("(\\d+) (blue|red|green)").unwrap();
            let captures = regex.captures_iter(line);
            for cap in captures {
                // Fun with match statements
                match (
                    cap.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                    cap.get(2).unwrap().as_str(),
                ) {
                    (n, "red") if n > 12 => return None,
                    (n, "green") if n > 13 => return None,
                    (n, "blue") if n > 14 => return None,
                    (_, _) => continue,
                }
            }

            Some((i + 1) as u32)
        })
        .sum();

    Some(answer)
}

// Practice using structs
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

// This is fun!
impl From<Cubes> for u32 {
    fn from(value: Cubes) -> Self {
        value.red * value.green * value.blue
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let answer = input
        .lines()
        .map(|line| {
            let mut cubes = Cubes {
                red: 0,
                green: 0,
                blue: 0,
            };
            let regex = Regex::new("(\\d+) (blue|red|green)").unwrap();
            let captures = regex.captures_iter(line);
            for cap in captures {
                match (
                    cap.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                    cap.get(2).unwrap().as_str(),
                ) {
                    (n, "red") => cubes.red = max(n, cubes.red),
                    (n, "green") => cubes.green = max(n, cubes.green),
                    (n, "blue") => cubes.blue = max(n, cubes.blue),
                    (_, _) => panic!(),
                }
            }

            // Can't find a way to have Rust infer the type :(
            u32::from(cubes)
        })
        .sum();

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
