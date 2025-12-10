#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let _ = input;
    Ok("{{project-name}} - part 1".to_string())
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
