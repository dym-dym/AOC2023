use std::collections::HashMap;

use rayon::{
    iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};

use crate::custom_error::AocError;

pub fn process(input: &str) -> miette::Result<i64, AocError> {
    let names = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    Ok(input
        .trim()
        .par_split('\n')
        .into_par_iter()
        .map(|elem: &str| {
            let mut values = vec![];

            for value in names.keys() {
                for (index, chain) in elem.match_indices(value) {
                    values.push((index, names[chain]));
                }
            }

            values.sort();
            10 * values[0].1 + values[values.len() - 1].1
        })
        .collect::<Vec<i64>>()
        .par_iter()
        .sum::<i64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(281, process(input)?);
        Ok(())
    }
}
