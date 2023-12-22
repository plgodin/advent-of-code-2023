use std::iter::repeat;
advent_of_code::solution!(12);

fn parse_input(input: &str, unfold: bool) -> Vec<(Vec<char>, Vec<u32>)> {
    input
        .lines()
        .map(|line| {
            let (mut springs, mut groups_str) = line.split_once(' ').unwrap();
            let rep_springs = (springs.to_owned() + "?").repeat(5);
            let rep_groups = (groups_str.to_owned() + ",").repeat(5);

            if unfold {
                springs = rep_springs.strip_suffix('?').unwrap();
                groups_str = rep_groups.strip_suffix(',').unwrap();
            }

            let groups = groups_str.split(',').map(|g| g.parse().unwrap());

            (springs.chars().collect(), groups.collect())
        })
        .collect()
}
pub fn part_one(input: &str) -> Option<u32> {
    let rows = parse_input(input, false);

    let answer = solve(rows);

    Some(answer)
}

fn fits_record(config: &Vec<u32>, record: &(Vec<char>, Vec<u32>)) -> bool {
    let guess = write_config(&record.1, config);
    let guess = guess.iter().chain(repeat(&'.'));

    record.0.iter().zip(guess).all(|(&r, &g)| match r {
        '?' => true,
        '.' => g == '.',
        '#' => g == '#',
        _ => panic!(),
    })
}

fn write_config(groups: &Vec<u32>, config: &Vec<u32>) -> Vec<char> {
    let mut out = Vec::new();
    for i in 0..groups.len() {
        out.extend(repeat('.').take(config[i] as usize));
        out.extend(repeat('#').take(groups[i] as usize));
        out.push('.');
    }
    out
}

fn get_possible_configs(groups: u32, max_offset: u32) -> Vec<Vec<u32>> {
    let mut all_configs = Vec::new();
    for first_offset in 0..=max_offset {
        if groups == 1 {
            all_configs.push(vec![first_offset]);
        } else {
            for mut config in get_possible_configs(groups - 1, max_offset - first_offset) {
                let mut new_config = vec![first_offset];
                new_config.append(&mut config);
                all_configs.push(new_config)
            }
        }
    }
    all_configs
}

pub fn part_two(input: &str) -> Option<u32> {
    let rows = parse_input(input, true);

    let answer = solve(rows);

    Some(answer)
}

fn solve(rows: Vec<(Vec<char>, Vec<u32>)>) -> u32 {
    rows.iter()
        .map(|row| {
            println!("Solving new line");
            let (springs, groups) = row;
            let min_length: u32 = groups.iter().sum::<u32>() + groups.len() as u32 - 1;
            let max_offset = springs.len() as u32 - min_length;

            let configs = get_possible_configs(groups.len() as u32, max_offset);

            configs.iter().filter(|&c| fits_record(c, row)).count() as u32
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
