advent_of_code::solution!(2);

pub fn is_valid_id(id: &u64) -> bool {
    let id_str = id.to_string();

    if id_str.len() %2 == 1 {
        return true;
    }

    for i in 0..(id_str.len()/2) {
        if id_str.chars().nth(i).unwrap() != id_str.chars().nth(i + id_str.len()/2).unwrap() {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
     let ranges : Vec<&str> = input.trim_ascii().split(",").collect();

     let mut result = 0;

     for r in ranges {
        dbg!(&r);
        let mut it = r.split("-").map(|x| x.parse::<u64>().unwrap());
        let min = it.next().unwrap();
        let max = it.next().unwrap();
        for id in min..=max {
            
            if !is_valid_id(&id) {
                println!("{}", id);    
                result += id;
            }
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
