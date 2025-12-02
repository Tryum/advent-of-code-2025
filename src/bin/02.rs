advent_of_code::solution!(2);

#[derive(PartialEq)]
pub enum Part {
    One,
    Two,
}

pub fn is_valid_id(id: &u64, part: &Part) -> bool {
    let id_str = id.to_string();

    if id_str.len() == 1 {
        return true;
    }

    let start_idx = match part {
        Part::One => id_str.len() / 2 - 1,
        Part::Two => 0,
    };

    if part == &Part::One && id_str.len().is_multiple_of(2) {
        return true;
    }

    for i in start_idx..(id_str.len() / 2) {
        let leftover = id_str.trim_start_matches(&id_str[0..=i]);
        if leftover.is_empty() {
            return false;
        }
    }

    true
}

fn solver(input: &str, part: &Part) -> Option<u64> {
    let ranges: Vec<&str> = input.trim_ascii().split(",").collect();

    let mut result = 0;

    for r in ranges {
        let mut it = r.split("-").map(|x| x.parse::<u64>().unwrap());
        let min = it.next().unwrap();
        let max = it.next().unwrap();
        for id in min..=max {
            if !is_valid_id(&id, part) {
                result += id;
            }
        }
    }

    Some(result)
}

pub fn part_one(input: &str) -> Option<u64> {
    solver(input, &Part::One)
}

pub fn part_two(input: &str) -> Option<u64> {
    solver(input, &Part::Two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }

    #[test]
    fn test_is_valid_id() {
        assert_eq!(is_valid_id(&1234, &Part::Two), true);
        assert_eq!(is_valid_id(&12341234, &Part::Two), false);
        assert_eq!(is_valid_id(&123123123, &Part::Two), false);
        assert_eq!(is_valid_id(&1212121212, &Part::Two), false);
        assert_eq!(is_valid_id(&1111111, &Part::Two), false);
        assert_eq!(is_valid_id(&38593862, &Part::Two), true);
        assert_eq!(is_valid_id(&1010, &Part::One), false);
    }
}
