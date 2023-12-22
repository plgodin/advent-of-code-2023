use num::abs;
advent_of_code::solution!(11);

fn parse_input(input: &str, expansion_factor: i64) -> Vec<(i64, i64)> {
    let mut vertical_expansion = 0;
    let mut galaxies = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            if !line.contains('#') {
                vertical_expansion += expansion_factor;
            }
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some((x as i64, y as i64 + vertical_expansion)),
                _ => None,
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    // Now add horizontal expansion
    let mut i = 0;
    while i < galaxies.iter().max_by_key(|&g| g.0).unwrap().0 {
        if galaxies.iter().any(|&g| g.0 == i) {
            i += 1;
            continue;
        }

        for g in galaxies.iter_mut().filter(|(x, _)| x > &i) {
            g.0 += expansion_factor;
        }

        i += 1 + expansion_factor;
    }

    galaxies
}

pub fn part_one(input: &str) -> Option<i64> {
    let galaxies = parse_input(input, 1);

    let mut total_length = 0;
    for i in 0..galaxies.len() {
        for g in galaxies.iter() {
            total_length += abs(g.0 - galaxies[i].0) + abs(g.1 - galaxies[i].1)
        }
    }

    Some(total_length / 2)
}

pub fn part_two(input: &str) -> Option<i64> {
    let galaxies = parse_input(input, 999_999);

    let mut total_length = 0;
    for i in 0..galaxies.len() {
        for g in galaxies.iter() {
            total_length += abs(g.0 - galaxies[i].0) + abs(g.1 - galaxies[i].1)
        }
    }

    Some(total_length / 2)
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
