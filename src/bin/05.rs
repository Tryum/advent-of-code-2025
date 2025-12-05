advent_of_code::solution!(5);

pub struct Input {
    pub fresh_ingredients: Vec<(u64, u64)>,
    pub ingredients: Vec<u64>,
}

pub enum ParserState {
    FreshIngredients,
    Ingredients,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut state = Input {
        fresh_ingredients: Vec::new(),
        ingredients: Vec::new(),
    };
    let mut parser_state = ParserState::FreshIngredients;
    for line in input.lines() {
        match parser_state {
            ParserState::FreshIngredients => {
                if line.trim().is_empty() {
                    parser_state = ParserState::Ingredients;
                } else {
                    let mut range = line.split('-').map(|x| x.parse::<u64>().unwrap());
                    let min = range.next().unwrap();
                    let max = range.next().unwrap();
                    state.fresh_ingredients.push((min, max));
                }
            }
            ParserState::Ingredients => {
                if line.trim().is_empty() {
                    break;
                }
                let ingredient = line.trim().parse::<u64>().unwrap();
                state.ingredients.push(ingredient);
            }
        }
    }

    let mut result = 0;

    for ingredient in state.ingredients {
        for (min, max) in &state.fresh_ingredients {
            if ingredient >= *min && ingredient <= *max {
                result += 1;
                break;
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
