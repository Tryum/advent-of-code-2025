advent_of_code::solution!(3);

pub fn largest_joltage_of_bank(input: &str, digit_count:  u64) -> Option<u64> {
    let mut result = 0;
    let mut last_index = 0;

    for i in 0..digit_count {
        let mut current_digit = 0;
        let remaining_digits = digit_count - i;
        for j in last_index..input.len()-(remaining_digits as usize)+1 {
            let c = input.chars().nth(j)?;
            let digit = c.to_digit(10)? as u64;
            if digit > current_digit {
                current_digit = digit;
                last_index = j;
                if current_digit == 9 {
                    break;
                }
            }
        }
        result = result * 10 + current_digit;
        last_index = last_index + 1;
    }
    Some(result)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if let Some(result) = largest_joltage_of_bank(line, 2) {
            total += result;
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if let Some(result) = largest_joltage_of_bank(line, 12) {
            total += result;
        }
    }
    Some(total)
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
        assert_eq!(result, Some(3121910778619));
    }

    #[test]
    fn test_largest_joltage_of_bank() {
        assert_eq!(largest_joltage_of_bank("987654321111111", 2), Some(98));
        assert_eq!(largest_joltage_of_bank("811111111111119", 2), Some(89));
        assert_eq!(largest_joltage_of_bank("234234234234278", 2), Some(78));
        assert_eq!(largest_joltage_of_bank("818181911112111", 2), Some(92));

        assert_eq!(largest_joltage_of_bank("987654321111111", 12), Some(987654321111));
        assert_eq!(largest_joltage_of_bank("811111111111119", 12), Some(811111111119));
        assert_eq!(largest_joltage_of_bank("234234234234278", 12), Some(434234234278));
        assert_eq!(largest_joltage_of_bank("818181911112111", 12), Some(888911112111));
    }
}
