use num::integer::lcm;
use std::collections::HashMap;
advent_of_code::solution!(8);

// struct Node<'a> {
//     id: u32,
//     left: &'a Node<'a>,
//     right: &'a Node<'a>,
// }
//
// impl From<Chars> for Node<'a> {
//     fn from(value: Chars) -> Self {}
// }

fn parse_node(id: &str) -> u32 {
    id.chars()
        .fold(0, |acc, c| (acc << 8) + c.to_digit(36).unwrap())
}

pub fn part_one(input: &str) -> Option<u32> {
    let (commands_vec, nodes) = parse_input(input);
    let mut commands = commands_vec.iter().cycle();

    let mut steps = 0;
    let mut current = nodes.get_key_value(&parse_node("AAA")).unwrap();

    while *current.0 != parse_node("ZZZ") {
        let next_node = match commands.next().unwrap() {
            'L' => current.1 .0,
            'R' => current.1 .1,
            _ => panic!(),
        };
        current = nodes.get_key_value(&next_node).unwrap();
        steps += 1;
    }

    Some(steps)
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<u32, (u32, u32)>) {
    let commands_vec = input.lines().next().unwrap().chars().collect::<Vec<_>>();

    let nodes: HashMap<u32, (u32, u32)> = input[2..]
        .lines()
        .map(|line| {
            if line.len() == 0 {
                return (0, (0, 0));
            };
            (
                parse_node(&line[0..3]),
                (parse_node(&line[7..10]), parse_node(&line[12..15])),
            )
        })
        .collect();
    (commands_vec, nodes)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (commands_vec, nodes) = parse_input(input);
    let mut commands = commands_vec.iter().cycle();

    // let linked_nodes: HashMap<_, _> = nodes
    //     .iter()
    //     .map(|(k, (l, r))| (k, (nodes.get(l).unwrap(), nodes.get(r).unwrap())))
    //     .collect();

    let mut currents = nodes
        .iter()
        .filter(|&node| *node.0 & 0xFF == 'A'.to_digit(36).unwrap())
        .collect::<Vec<_>>();

    let all_steps: Vec<_> = currents
        .iter()
        .map(|&node| {
            let mut steps = 0;
            let mut current = node.clone();
            while *current.0 & 0xFF != 'Z'.to_digit(36).unwrap() {
                let next_node = match commands.next().unwrap() {
                    'L' => current.1 .0,
                    'R' => current.1 .1,
                    _ => panic!(),
                };
                current = nodes.get_key_value(&next_node).unwrap();
                steps += 1;
            }
            steps
        })
        .collect();

    println!("Done finding cycles!");

    let answer = all_steps.iter().fold(1, |acc, steps| lcm(acc, *steps));
    // while !currents
    //     .iter()
    //     .all(|&node| *node.0 & 0xFF == 'Z'.to_digit(36).unwrap())
    // {
    //     let next_nodes: Vec<_> = match commands.next().unwrap() {
    //         'L' => currents.iter().map(|&node| (node.1).0).collect(),
    //         'R' => currents.iter().map(|&node| (node.1).1).collect(),
    //         _ => panic!(),
    //     };
    //     // println!(
    //     //     "Current nodes are {}, {}, {}, {}, {}, {}",
    //     //     currents[0].0 & 0xFF,
    //     //     currents[1].0 & 0xFF,
    //     //     currents[2].0 & 0xFF,
    //     //     currents[3].0 & 0xFF,
    //     //     currents[4].0 & 0xFF,
    //     //     currents[5].0 & 0xFF
    //     // );
    //     currents = next_nodes
    //         .iter()
    //         .map(|&node| nodes.get_key_value(&node).unwrap())
    //         .collect();
    //     steps += 1;
    //     // if steps > 4 {
    //     //     panic!();
    //     // }
    //     if steps % 100000000 == 0 {
    //         println!("We ran {} steps yet...", steps);
    //     }
    // }
    // while *current.0 != parse_node("ZZZ") {
    //     let next_node = match commands.next().unwrap() {
    //         'L' => current.1 .0,
    //         'R' => current.1 .1,
    //         _ => panic!(),
    //     };
    //     current = nodes.get_key_value(&next_node).unwrap();
    //     steps += 1;
    // }

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
