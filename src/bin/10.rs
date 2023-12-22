advent_of_code::solution!(10);

#[derive(PartialEq, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn flip(&self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

struct Pipe(Dir, Dir);

impl Pipe {
    fn follow(&self, from: Dir) -> &Dir {
        if from == self.0 {
            &self.1
        } else {
            &self.0
        }
    }
}

impl PartialEq for Pipe {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

fn parse_input(input: &str) -> ((usize, usize), Vec<Vec<Pipe>>) {
    let mut start_point = (0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    match c {
                        '-' => Pipe(Dir::Left, Dir::Right),
                        '|' => Pipe(Dir::Up, Dir::Down),
                        'F' => Pipe(Dir::Down, Dir::Right),
                        'J' => Pipe(Dir::Up, Dir::Left),
                        'L' => Pipe(Dir::Up, Dir::Right),
                        '7' => Pipe(Dir::Left, Dir::Down),
                        'S' => {
                            start_point = (x, y);
                            Pipe(Dir::Left, Dir::Down)
                        } // By observing input
                        '.' => Pipe(Dir::Down, Dir::Down), // Don't care
                        _ => panic!(),
                    }
                })
                .collect()
        })
        .collect();

    (start_point, map)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (start_point, map) = parse_input(input);

    let mut path = vec![start_point];
    let mut current = start_point;
    let mut next_dir = Dir::Left;

    while *path.first().unwrap() != current || path.len() == 1 {
        current = match next_dir {
            Dir::Up => (current.0, current.1 - 1),
            Dir::Down => (current.0, current.1 + 1),
            Dir::Left => (current.0 - 1, current.1),
            Dir::Right => (current.0 + 1, current.1),
        };
        next_dir = *map[current.1][current.0].follow(next_dir.flip());
        path.push(current);
    }

    Some((path.len() / 2) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (start_point, map) = parse_input(input);

    let mut path = vec![(start_point, Dir::Left)];
    let mut current = start_point;
    let mut next_dir = Dir::Left;

    while path.first().unwrap().0 != current || path.len() == 1 {
        current = match next_dir {
            Dir::Up => (current.0, current.1 - 1),
            Dir::Down => (current.0, current.1 + 1),
            Dir::Left => (current.0 - 1, current.1),
            Dir::Right => (current.0 + 1, current.1),
        };
        next_dir = *map[current.1][current.0].follow(next_dir.flip());
        path.push((current, next_dir));
    }

    let mut count_in = 0;

    let tiles = map
        .iter()
        .enumerate()
        .map(|(y, line)| line.iter().enumerate().map(move |(x, pipe)| ((x, y), pipe)))
        .flatten();

    'tiles: for tile in tiles {
        if path.iter().any(|&step| step.0 == tile.0) {
            // tile is already on path
            continue;
        }
        let mut current_x = tile.0 .0;
        'walk: while !path.iter().any(|x| x.0 == (current_x, tile.0 .1)) {
            if current_x == map[0].len() {
                continue 'tiles;
            } // tile has line of sight with right edge
            current_x += 1;
        }
        // tile has line of sight with path
        match path
            .iter()
            .enumerate()
            .find_map(|(i, ((x, y), dir))| {
                if (x, y) == (&current_x, &tile.0 .1) {
                    Some((i, dir))
                } else {
                    None
                }
            })
            .unwrap()
        {
            (_, Dir::Up) => {
                // Might be Dir::Up
                count_in += 1;
            }
            (i, Dir::Right) => {
                if path[i - 1].1 == Dir::Up {
                    count_in += 1;
                }
            }
            _ => {}
        }
    }

    Some(count_in)
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
