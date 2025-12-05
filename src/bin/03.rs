advent_of_code::solution!(3);

pub fn largest_joltage_of_bank(input: &str) -> Option<u64> {
    let mut first_digit = 0;
    let mut first_digit_index = 0;
    let mut second_digit = 0;
    for i in 0..input.len() - 1 {
        let c = input.chars().nth(i)?;
        let digit = c.to_digit(10)? as u64;
        if digit > first_digit {
            first_digit = digit;
            first_digit_index = i;
        }
    }

    for i in first_digit_index + 1..input.len() {
        let c = input.chars().nth(i)?;
        let digit = c.to_digit(10)? as u64;
        if  digit > second_digit {
            second_digit = digit;
        }
    }

    Some(first_digit * 10 + second_digit)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if let Some(result) = largest_joltage_of_bank(line) {
            total += result;
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_largest_joltage_of_bank() {
        assert_eq!(largest_joltage_of_bank("987654321111111"), Some(98));
        assert_eq!(largest_joltage_of_bank("811111111111119"), Some(89));
        assert_eq!(largest_joltage_of_bank("234234234234278"), Some(78));
        assert_eq!(largest_joltage_of_bank("818181911112111"), Some(92));
    }
}
