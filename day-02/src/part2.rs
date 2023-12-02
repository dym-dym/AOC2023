use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    str::ParallelString,
};

use crate::custom_error::AocError;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let x: u32 = input
        .par_lines()
        .into_par_iter()
        .map(|line| {
            let mut reds_amount = vec![];
            let mut greens_amount = vec![];
            let mut blues_amount = vec![];

            let sub_games = line.trim().split(':').collect::<Vec<&str>>()[1].split(';');

            for game in sub_games {
                for value in game.trim().split(',') {
                    match value
                        .trim()
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .as_slice()
                    {
                        [x, "red"] => reds_amount.push(x.parse::<u32>().unwrap()),
                        [x, "green"] => greens_amount.push(x.parse::<u32>().unwrap()),
                        [x, "blue"] => blues_amount.push(x.parse::<u32>().unwrap()),
                        _ => (),
                    }
                }
            }

            reds_amount.into_iter().max().unwrap()
                * greens_amount.into_iter().max().unwrap()
                * blues_amount.into_iter().max().unwrap()
        })
        .collect::<Vec<u32>>()
        .into_par_iter()
        .sum();

    Ok(x.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}
