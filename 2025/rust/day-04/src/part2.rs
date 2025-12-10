#[tracing::instrument(skip(input))]
pub fn process(_input: &str) -> miette::Result<String> {
    Ok("day-04 - part 2".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
