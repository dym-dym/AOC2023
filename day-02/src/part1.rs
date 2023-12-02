use crate::custom_error::AocError;

const AVAILABLE_RED_CUBES: u16 = 12;
const AVAILABLE_GREEN_CUBES: u16 = 13;
const AVAILABLE_BLUE_CUBES: u16 = 14;

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let x: u16 = input
        .lines()
        .into_iter()
        .map(|line| {
            let false_number = line.trim().split(':').collect::<Vec<&str>>()[1]
                .split(';')
                .map(|sub_game| {
                    let mut current_red = 0;
                    let mut current_green = 0;
                    let mut current_blue = 0;

                    for value in sub_game.trim().split(',') {
                        match value
                            .trim()
                            .split_whitespace()
                            .collect::<Vec<&str>>()
                            .as_slice()
                        {
                            [x, "red"] => current_red = current_red + x.parse::<u16>().unwrap(),
                            [x, "green"] => {
                                current_green = current_green + x.parse::<u16>().unwrap()
                            }
                            [x, "blue"] => current_blue = current_blue + x.parse::<u16>().unwrap(),
                            _ => (),
                        }
                    }

                    current_red <= AVAILABLE_RED_CUBES
                        && current_green <= AVAILABLE_GREEN_CUBES
                        && current_blue <= AVAILABLE_BLUE_CUBES
                })
                .collect::<Vec<bool>>()
                .into_iter()
                .filter(|x| !x)
                .count();

            false_number != 0
        })
        .enumerate()
        .filter(|x| !x.1)
        .map(|x| (x.0 + 1) as u16)
        .collect::<Vec<u16>>()
        .into_iter()
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
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
