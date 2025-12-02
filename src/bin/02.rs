advent_of_code::solution!(2);

pub fn is_valid_id(id: &u64) -> bool {
    let id_str = id.to_string();

    for i in 0..(id_str.len()/2) {
        let leftover = id_str.trim_start_matches(&id_str[0..=i]);
        if leftover.len() == 0 {
            return false;
        }
    }

    true
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
        assert_eq!(is_valid_id(&1234), true);
        assert_eq!(is_valid_id(&12341234), false);
        assert_eq!(is_valid_id(&123123123), false);
        assert_eq!(is_valid_id(&1212121212), false);
        assert_eq!(is_valid_id(&1111111), false);
        assert_eq!(is_valid_id(&38593862), true);
    }
}
