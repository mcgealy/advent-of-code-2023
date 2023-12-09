use std::vec;

advent_of_code::solution!(1);

fn parse(input: &str) -> u32 {
    let parsed: Vec<u32> = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    10 * parsed.first().unwrap() + parsed.last().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input
        .lines()
        .map(parse)
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input
        .lines()
        .map(|line| {
            line
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
        })
        .map(|l| parse(&l))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(363));
    }
}
