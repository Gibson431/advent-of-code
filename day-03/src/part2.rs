pub fn process(_input: &str) -> String {
    todo!("part 2");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("", "")]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process(input));
        todo!("haven't built test yet");
    }
}
