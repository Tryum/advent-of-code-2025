advent_of_code::solution!(1);

const DIAL_MAX : u64 = 100;

pub fn part_one(input: &str) -> Option<u64> {
    let mut index = 50;
    let mut password = 0;

    println!("- The dial starts by pointing at {}.", index);

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        print!("- The dial is rotated {}", line);
        let dir = &line[0..1];
        let dist: u64 = line[1..].parse::<u64>().unwrap() % DIAL_MAX;
        dbg!(dist);
        dbg!(dist < DIAL_MAX);
        match dir {
            "R" => index += dist,
            "L" => index += DIAL_MAX - dist,
            _ => panic!("Invalid direction"),
        }
        index %= DIAL_MAX;

        println!(" to point at {}.", index);

        
        if index == 0 {
            password += 1;
        }
    }
    Some(password)
}

pub fn part_two(input: &str) -> Option<u64> {
        let mut index = 50;
    let mut password = 0;

    println!("- The dial starts by pointing at {}.", index);

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        print!("- The dial is rotated {}", line);
        let dir = &line[0..1];
        let dist: u64 = line[1..].parse::<u64>().unwrap();
        password += dist/DIAL_MAX;
        let dist = dist % DIAL_MAX;
        match dir {
            "R" => {
                if index + dist > DIAL_MAX {
                    println!(" it points at 0 once.");
                    password += 1;
                }
                index += dist
            },
            "L" =>{
                if dist > index  && index != 0 {
                    println!(" it points at 0 once.");
                    password += 1;
                }
                index += DIAL_MAX - dist
            },
            _ => panic!("Invalid direction"),
        }
        index %= DIAL_MAX;

        println!(" to point at {}.", index);

        
        if index == 0 {
            password += 1;
        }
    }
    Some(password)
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
