advent_of_code::solution!(6);

pub fn win(hold_time: u32, race_time: u32, record: u32) -> bool {
    let moving_time = race_time - hold_time;

    hold_time * moving_time > record
}

pub fn part_one(input: &str) -> Option<u32> {
    const RACES: [(u32, u32); 4] = [(46, 347), (82, 1522), (84, 1406), (79, 1471)];

    // distance = speed * moving_time;
    // distance = (hold_time) * moving_time;
    // moving_time + hold_time = race_time
    //
    // distance > record
    // hold_time * moving_time = record
    // hold_time * moving_time = race_time

    let mut total_win = 1;
    for (time, record) in RACES {
        let mut win_count = 0;
        for hold in 0..time {
            if (win(hold, time, record)) {
                win_count += 1;
            }
        }
        total_win *= win_count;
    }

    Some(total_win)
}

pub fn part_two(input: &str) -> Option<u32> {
    // hold * move = 347152214061471;
    // hold + move = 46828479;
    // move = 46828479 - hold;
    // hold * (46828479 - hold) = 347152214061471;
    // -hold^2 + 46828479hold - 347152214061471 = 0;
    None
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
