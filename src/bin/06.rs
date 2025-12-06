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

pub fn parse(input: &str) -> Vec<Problem> {
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

    problems
}

pub fn parse2(input: &str) -> Vec<Problem> {
    let mut problems = Vec::<Problem>::new();
    let length = input.lines().next().unwrap().len();

    let lines = input.lines().collect::<Vec<&str>>();

    let mut problem = Problem::default();
    for i in 0..length {
        let mut number = 0;
        let mut digit = 0;
        for l in &lines {
            let c = l.chars().nth(i).unwrap();
            match c {
                '*' | '+' => match c {
                    '*' => problem.operation = Some(Operation::Multiply),
                    '+' => problem.operation = Some(Operation::Add),
                    _ => {}
                },
                ' ' => {}
                _ => {
                    let n = c.to_string().parse::<u64>().unwrap();
                    number = number * 10 + n;
                    digit += 1;
                }
            }
        }
        if digit > 0 {
            problem.numbers.push(number);
        } else {
            problems.push(problem);
            problem = Problem::default();
        }
    }
    if !problem.numbers.is_empty() {
        problems.push(problem);
    }

    problems
}

pub fn part_one(input: &str) -> Option<u64> {
    let problems = parse(input);

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
    let problems = parse2(input);

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
        assert_eq!(result, Some(3263827));
    }
}
