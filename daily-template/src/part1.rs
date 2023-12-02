pub fn process(_input: &str) -> &str {
    todo!("part 1");
}

fn process_line(_input: &str) -> &str {
    todo!("part 1");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("", "")]
    fn test_process_line(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process_line(input));
    }

    #[test]
    fn test_process() {
        let input = "";
        assert_eq!("", process(input));
        todo!("haven't built test yet");
    }
}
