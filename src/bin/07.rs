use std::{collections::HashSet, mem::swap};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut count = 0;

    let mut lines = input.lines();

    let first_line = lines.next().unwrap_or("");
    let first_beam = first_line.find('S').unwrap();

    let mut beams = HashSet::new();
    beams.insert(first_beam);
    let mut next_beams = HashSet::new();

    for line in lines {
        for beam in beams.drain() {
            if line.chars().nth(beam) == Some('^') {
                count += 1;
                if beam > 0 {
                    next_beams.insert(beam - 1);
                }
                next_beams.insert(beam + 1);
            } else {
                next_beams.insert(beam);
            }
        }
        swap(&mut beams, &mut next_beams);
    }

    Some(count)
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
