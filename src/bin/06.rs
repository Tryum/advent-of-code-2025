advent_of_code::solution!(6);

pub enum Operation {
    Add,
    Multiply,
}

#[derive(Default)]
pub struct Problem {
    pub operation: Option<Operation>,
    pub numbers: Vec<u64>,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut problems = Vec::<Problem>::new();

    for l in input.lines() {
        if l.trim().is_empty() {
            break;
        }
        match l.chars().next() {
            Some('*') | Some('+') => {
                for (idx, op) in l.split_ascii_whitespace().enumerate() {
                    match op {
                        "*" => problems[idx].operation = Some(Operation::Multiply),
                        "+" => problems[idx].operation = Some(Operation::Add),
                        _ => {}
                    }
                }
            }
            _ => {
                let numbers: Vec<u64> = l
                    .split_ascii_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect();
                if numbers.len() != problems.len() {
                    problems.resize_with(numbers.len(), Problem::default);
                }
                for (idx, n) in numbers.iter().enumerate() {
                    problems[idx].numbers.push(*n);
                }
            }
        }
    }

    let mut result = 0;
    for p in problems {
        match p.operation {
            Some(Operation::Add) => {
                let sum: u64 = p.numbers.iter().sum();
                result += sum;
            }
            Some(Operation::Multiply) => {
                let product: u64 = p.numbers.iter().product();
                result += product;
            }
            None => {}
        }
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
