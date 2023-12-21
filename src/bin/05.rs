advent_of_code::solution!(5);

struct MapRange {
    src: u64,
    dst: u64,
    len: u64,
}

struct Map(Vec<MapRange>);

impl Map {
    fn map(&self, n: u64) -> u64 {
        self.0
            .iter()
            .find_map(|map| {
                if ((map.src)..(map.src + map.len)).contains(&n) {
                    Some(map.dst + n - map.src)
                } else {
                    None
                }
            })
            .unwrap_or(n)
    }
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<Map>) {
    let mut input_iter = input.lines();
    let seed_strs = input_iter.next().unwrap().strip_prefix("seeds: ").unwrap();

    let seeds = seed_strs
        .split_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let map_ranges = input_iter.map(|range_str| {
        let mut range_params = range_str.split_whitespace();
        Some(MapRange {
            dst: range_params.next()?.parse().ok()?,
            src: range_params.next()?.parse().ok()?,
            len: range_params.next()?.parse().ok()?,
        })
    });

    let maps = map_ranges.fold(Vec::new(), |mut acc, mr| {
        match mr {
            None => {
                acc.push(Map(Vec::new()));
            }
            Some(range) => {
                acc.last_mut().unwrap().0.push(range);
            }
        }
        acc
    });

    let maps = maps.into_iter().filter(|x| !x.0.is_empty()).collect();

    (seeds, maps)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (seeds, maps) = parse_input(input);

    let answer = seeds
        .iter()
        .map(|&seed| maps.iter().fold(seed, |acc, map| map.map(acc)))
        .min();

    u32::try_from(answer.unwrap()).ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (seeds, maps) = parse_input(input);

    let all_seeds = seeds
        .chunks(2)
        .flat_map(|range| range[0]..(range[0] + range[1]))
        .collect::<Vec<_>>();

    println!("We have {} seeds and {} maps", all_seeds.len(), maps.len());

    // Interestingly, the approach below gives approximately the same performance!
    // let answer = maps
    //     .into_iter()
    //     .fold(all_seeds, |acc, map| {
    //         println!("Starting new map!");
    //         acc.into_iter().map(|seed| map.map(seed)).collect()
    //     })
    //     .into_iter()
    //     .min();

    let answer = all_seeds
        .into_iter()
        .map(|seed| maps.iter().fold(seed, |acc, map| map.map(acc)))
        .min();

    u32::try_from(answer.unwrap()).ok()
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
