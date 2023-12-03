use itertools::Itertools;

use crate::custom_error::AocError;

#[derive(Debug, Clone)]
enum ValueType {
    Number(u8),
    Empty,
    Symbol,
}

#[derive(Debug, Clone)]
struct CharType {
    value: ValueType,
    keep_group: bool,
    coordinates: (usize, usize),
}

fn filter_value(x: usize, y: usize, character: &char) -> CharType {
    CharType {
        coordinates: (x, y),
        value: match character {
            c if c.is_digit(10) => ValueType::Number(
                character
                    .to_string()
                    .parse::<u8>()
                    .expect("Should be a number"),
            ),
            '.' => ValueType::Empty,
            _ => ValueType::Symbol,
        },
        keep_group: false,
    }
}

fn filter_keepable(character: &CharType, symbol_vec: &Vec<(usize, usize)>) -> CharType {
    let x = character.coordinates.0 as i32;
    let y = character.coordinates.1 as i32;

    let window = vec![
        (x, y + 1),
        (x, y - 1),
        (x + 1, y),
        (x - 1, y),
        (x + 1, y + 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
        (x - 1, y - 1),
    ];

    for (x, y) in window {
        if symbol_vec.contains(&(x as usize, y as usize)) {
            return CharType {
                value: character.value.clone(),
                coordinates: character.coordinates,
                keep_group: true,
            };
        }
    }
    character.clone()
}

fn do_keep_group(group: &Vec<&CharType>) -> bool {
    group
        .iter()
        .fold(false, |acc, character| acc || character.keep_group)
}

fn group_integer(group: &Vec<&CharType>) -> u32 {
    group
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, character)| {
            if let ValueType::Number(value) = character.value {
                acc + (10 as u32).pow(index as u32) * value as u32
            } else {
                acc + 0
            }
        })
}

fn flatten_groups(groups: &Vec<Vec<&CharType>>) -> u32 {
    let mut res = 0;
    for group in groups.iter() {
        if do_keep_group(group) {
            res = res + group_integer(group);
        }
    }
    res
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let grid = input
        .trim()
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|x| char::from_u32(*x as u32).expect("Should be a char"))
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let processed_characters = grid
        .iter()
        .enumerate()
        .map(|(x, line)| {
            line.iter()
                .enumerate()
                .map(|(y, character)| filter_value(x, y, character))
                .collect::<Vec<CharType>>()
        })
        .collect::<Vec<Vec<CharType>>>();

    let characters = processed_characters
        .iter()
        .flatten()
        .collect::<Vec<&CharType>>();

    let numbers = characters
        .iter()
        .filter(|x| matches!(x.value, ValueType::Number(_)))
        .collect::<Vec<_>>();

    let symbols = characters
        .clone()
        .iter()
        .filter(|x| matches!(x.value, ValueType::Symbol))
        .map(|x| x.coordinates)
        .collect::<Vec<(usize, usize)>>();

    let associated_numbers = numbers
        .into_iter()
        .map(|number| filter_keepable(number, &symbols))
        .collect::<Vec<CharType>>();

    let mut res = 0;

    for (_, groups) in &associated_numbers
        .into_iter()
        .group_by(|elt| elt.coordinates.0)
    {
        let groups = groups.collect::<Vec<_>>();
        let groups = (&(0..groups.len()).group_by(|&i| groups[i].coordinates.1 as usize - i))
            .into_iter()
            .map(|(_, group)| group.map(|i| &groups[i]).collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>();

        res = res + flatten_groups(&groups);
    }

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
