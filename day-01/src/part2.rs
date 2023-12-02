use crate::custom_error::AocError;

pub fn process(input: &str) -> miette::Result<i64, AocError> {
    let names = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let replace_by_number = |value| names.iter().position(|x| x == &value).unwrap() + 1;

    Ok(input
        .trim()
        .split("\n")
        .map(|elem: &str| {
            let mut values = vec![];

            for value in &names {
                for (index, chain) in elem.match_indices(value) {
                    values.push((index, replace_by_number(chain) as i64));
                }
            }

            for (index, character) in elem.chars().enumerate() {
                if character.is_numeric() {
                    values.push((index, character.to_string().parse::<i64>().unwrap()));
                }
            }

            values.sort();
            10 * values[0].1 + values[values.len() - 1].1
        })
        .collect::<Vec<i64>>()
        .iter()
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
