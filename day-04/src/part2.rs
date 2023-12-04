use crate::custom_error::AocError;

fn get_list_pairs(string: Vec<&str>) -> Vec<Vec<u32>> {
    let list1 = string[0]
        .trim()
        .split_whitespace()
        .map(|elt| elt.parse::<u32>().expect("Should be a number"))
        .collect();
    let list2 = string[1]
        .trim()
        .split_whitespace()
        .map(|elt| elt.parse::<u32>().expect("Should be a number"))
        .collect();

    vec![list1, list2]
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let res = input
        .trim()
        .lines()
        .map(|line| line.trim().split(':').collect::<Vec<&str>>()[1])
        .map(|game| { game.trim().split('|') }.collect::<Vec<&str>>())
        .map(|string| get_list_pairs(string))
        .map(|pair| pair[1].iter().filter(|x| pair[0].contains(x)).count() as u32)
        .inspect(|x| println!("matches : {x}"))
        .fold(0, |acc: u32, x: u32| {
            let n = acc + if x > 0 { (2 as u32).pow(x - 1) } else { 0 };
            println!("Score : {n}");
            n
        });

    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("13", process(input)?);
        Ok(())
    }
}
