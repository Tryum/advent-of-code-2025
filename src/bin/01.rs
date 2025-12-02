advent_of_code::solution!(1);

const DIAL_MAX: u64 = 100;

#[derive(PartialEq)]
pub enum Part {
    One,
    Two,
}

fn solve(input: &str, part: Part) -> Option<u64> {
    let mut index = 50;
    let mut password = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let dir = &line[0..1];
        let dist: u64 = line[1..].parse::<u64>().unwrap();
        if part == Part::Two {
            password += dist / DIAL_MAX;
        }

        let dist = dist % DIAL_MAX;
        match dir {
            "R" => {
                if index + dist > DIAL_MAX && part == Part::Two {
                    password += 1;
                }
                index += dist
            }
            "L" => {
                if dist > index && index != 0 && part == Part::Two {
                    password += 1;
                }
                index += DIAL_MAX - dist
            }
            _ => panic!("Invalid direction"),
        }
        index %= DIAL_MAX;

        if index == 0 {
            password += 1;
        }
    }
    Some(password)
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, Part::One)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, Part::Two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
