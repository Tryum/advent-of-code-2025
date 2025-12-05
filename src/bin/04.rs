advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    let mut grid = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        grid.push(line.chars().collect::<Vec<char>>());
    }

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '@' {
                let mut count = 0;
                for di in -1..=1 {
                    for dj in -1..=1 {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        let ni = i as isize + di;
                        let nj = j as isize + dj;
                        if ni >= 0
                            && ni < grid.len() as isize
                            && nj >= 0
                            && nj < grid[0].len() as isize
                            && grid[ni as usize][nj as usize] == '@'
                        {
                            count += 1;
                        }
                    }
                }
                if count < 4 {
                    result += 1;
                    print!("X");
                } else {
                    print!("@");
                }
            } else {
                print!(".");
            }
        }
        println!()
    }
    Some(result)
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
