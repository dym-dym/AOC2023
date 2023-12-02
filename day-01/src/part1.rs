use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::custom_error::AocError;

pub fn process(input: &str) -> miette::Result<i64, AocError> {
    Ok(input
        .trim_end()
        .split('\n')
        .map(|x| {
            x.trim_end()
                .chars()
                .filter(|x| x.is_numeric())
                .collect::<Vec<char>>()
        })
        .map(|x| {
            let mut res = String::from(x[0]);
            res.push(x[x.len() - 1]);
            res.parse::<i64>().unwrap()
        })
        .collect::<Vec<i64>>()
        .par_iter()
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(142, process(input)?);
        Ok(())
    }
}
