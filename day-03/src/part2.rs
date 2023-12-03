use std::usize;

use itertools::Itertools;

use crate::custom_error::AocError;

#[derive(Debug, Clone)]
enum ValueType {
    Number(u8),
    Empty,
    Gear(((usize, usize), (usize, usize))),
}

#[derive(Debug, Clone)]
struct GearPart {
    value: u32,
    gear: (usize, usize),
}

#[derive(Debug, Clone)]
struct CharType {
    value: ValueType,
    coordinates: (usize, usize),
    related_gear: (usize, usize),
}

// Constructs a CharType off of a character and its coordinates in the grid
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
            '*' => ValueType::Gear(((0, 0), (0, 0))),
            _ => ValueType::Empty,
        },
        related_gear: (0, 0),
    }
}

// Computes if a character has a gear symbol in its surrounding
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

    for (xi, yi) in window {
        if symbol_vec.contains(&(xi as usize, yi as usize)) {
            return CharType {
                value: character.value.clone(),
                coordinates: character.coordinates,
                related_gear: (xi as usize, yi as usize),
            };
        }
    }
    character.clone()
}

// gives back true if any number in the integer chain is true
fn do_keep_integer(integer: &Vec<&CharType>) -> bool {
    integer.iter().fold(false, |acc, character| {
        acc || character.related_gear != (0, 0)
    })
}

// Computes the value of a chain of integer
fn group_integer(group: &Vec<&CharType>) -> u32 {
    group
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, character)| {
            if let ValueType::Number(value) = character.value {
                acc + (10 as u32).pow(index as u32) * value as u32
            } else {
                acc
            }
        })
}

fn max_group_gear(group: &Vec<&CharType>) -> (usize, usize) {
    let res = group
        .iter()
        .max_by_key(|elt| elt.related_gear)
        .expect("Should be a CharType")
        .related_gear;
    res
}

// Flattens groups of integers (a.k.a a line) and gives back their sum
fn flatten_groups(groups: &Vec<Vec<&CharType>>) -> Vec<GearPart> {
    let mut integers = vec![];

    for group in groups.iter() {
        if do_keep_integer(group) {
            integers.push(GearPart {
                value: group_integer(group),
                gear: max_group_gear(group),
            });
        }
    }
    integers
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    // Process the grid as a 2d vector of characters
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

    // Make that grid into a 2d vector of an easier to process custom type
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

    // Flattens the 2d vector because coordinates are now encoded into a tuple linked to the value
    // of the character
    let characters = processed_characters
        .iter()
        .flatten()
        .collect::<Vec<&CharType>>();

    // Gets only the characters that are numbers
    let numbers = characters
        .iter()
        .filter(|x| matches!(x.value, ValueType::Number(_)))
        .collect::<Vec<_>>();

    // Gets only the characters that are Symbols
    let symbols = characters
        .clone()
        .iter()
        .filter(|x| matches!(x.value, ValueType::Gear((_, _))))
        .map(|x| x.coordinates)
        .collect::<Vec<(usize, usize)>>();

    // Processes the numbers to find out which one are next to symbols
    let associated_numbers = numbers
        .into_iter()
        .map(|number| filter_keepable(number, &symbols))
        .collect::<Vec<CharType>>();

    let mut final_process = vec![];

    // Groups lines together
    for (_, groups) in &associated_numbers
        .into_iter()
        .group_by(|elt| elt.coordinates.0)
    {
        let groups = groups.collect::<Vec<CharType>>();

        // Groups contiguous strings of numbers together
        let groups = (&(0..groups.len()).group_by(|&i| groups[i].coordinates.1 as usize - i))
            .into_iter()
            .map(|(_, group)| group.map(|i| &groups[i]).collect::<Vec<&CharType>>())
            .collect::<Vec<Vec<&CharType>>>();

        // Processes the value of each string of number in the line and sums them up
        final_process.push(flatten_groups(&groups));
    }

    let mut res = 0;

    for (_, vectors) in &final_process
        .iter()
        .flatten()
        .sorted_by(|a, b| Ord::cmp(&a.gear, &b.gear))
        .group_by(|elt| elt.gear)
    {
        let vectors = vectors.collect::<Vec<_>>();
        if vectors.len() >= 2 {
            res = res + (vectors[0].value * vectors[1].value);
        }
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
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}
