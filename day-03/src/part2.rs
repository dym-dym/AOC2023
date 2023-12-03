use crate::custom_error::AocError;

pub fn process(_input: &str) -> miette::Result<String, AocError> {
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
