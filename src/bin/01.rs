advent_of_code::solution!(1);

// It's day one, let's use an easy for loop to warm up
pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let mut chars = line.chars().filter_map(|c| c.to_digit(10));

        let first_digit = chars.next()?;
        let last_digit = chars.last().unwrap_or(first_digit);

        sum += first_digit * 10 + last_digit;
    }

    Some(sum)
}

// Now let's go full iterator
pub fn part_two(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|line| {
            let first_digit = find_digit(line, line.chars().enumerate().collect()).unwrap();

            let mut vector: Vec<_> = line.chars().enumerate().collect();
            vector.reverse();

            let second_digit = find_digit(line, vector).unwrap();
            first_digit * 10 + second_digit
        })
        .sum();

    Some(sum)
}

// A bit wonky, but it works for finding numbers in both directions...
fn find_digit(line: &str, chars: Vec<(usize, char)>) -> Option<u32> {
    chars.iter().find_map(|(i, char)| {
        if let Some(digit) = char.to_digit(10) {
            Some(digit)
        } else if let Some(numeral) = get_numeral(&line[*i..]) {
            Some(numeral)
        } else {
            None
        }
    })
}

fn get_numeral(input: &str) -> Option<u32> {
    static NUMERALS: &[&str] = &[
        "_",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];

    for (value, numeral) in NUMERALS.iter().enumerate() {
        if input.starts_with(numeral) {
            return Some(value.try_into().unwrap());
        }
    }

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
