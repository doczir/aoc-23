advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input.lines().map(|line| get_num(line)).sum();
    Some(result)
}

fn get_num(line: &str) -> u32 {
    let mut first: u32 = 0;
    let mut last: u32 = 0;

    for c in line.chars() {
        if c.is_digit(10) {
            first = c.to_digit(10).unwrap();
            break;
        }
    }

    for c in line.chars().rev() {
        if c.is_digit(10) {
            last = c.to_digit(10).unwrap();
            break;
        }
    }

    first * 10 + last
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|line| get_digits(line))
        .map(|digits| digits.first().unwrap() * 10 + digits.last().unwrap())
        .sum();
    Some(result)
}

const NUMBERS: [(&'static str, u32, usize); 9] = [
    ("one", 1, 1),
    ("two", 2, 1),
    ("six", 6, 1),
    ("four", 4, 1),
    ("five", 5, 1),
    ("nine", 9, 1),
    ("three", 3, 1),
    ("seven", 7, 1),
    ("eight", 8, 1),
];

fn get_digits(line: &str) -> Vec<u32> {
    let chars: Vec<_> = line.chars().collect();

    let mut digits = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_digit(10) {
            digits.push(chars[i].to_digit(10).unwrap());
            i += 1;
            continue;
        }

        for (name, digit, skip) in NUMBERS {
            if chars.len() < i + name.len() {
                break;
            }
            let str: String = chars[i..i + name.len()].iter().collect();
            if str == name {
                digits.push(digit);
                i += skip - 1;
                break;
            }
        }
        i += 1;
    }

    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
