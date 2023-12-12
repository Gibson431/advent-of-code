use nom::{
    bytes::complete::tag,
    character::complete::{self, space1},
    multi::separated_list1,
    IResult,
};

fn parse_line(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, list) = separated_list1(space1, complete::i32)(input)?;
    Ok((input, list))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    let (input, list) = separated_list1(tag("\n"), parse_line)(input)?;
    Ok((input, list))
}

fn process_history(input: &Vec<i32>) -> i32 {
    todo!()
}

pub fn process(input: &str) -> String {
    let (_, histories) = parse_input(input).expect("should succeed");

    histories.iter().fold(0, |acc, history| {
        acc + process_history(history)
    }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("0 3 6 9 12 15", vec![0,3,6,9,12,15])]
    #[case("1 3 6 10 15 21", vec![1,3,6,10,15,21])]
    #[case("10 13 16 21 30 45", vec![10,13,16,21,30,45])]
    fn test_parse_line(#[case] input: &str, #[case] expected: Vec<i32>) {
        let (_, res) = parse_line(input).expect("should work");
        assert_eq!(expected, res);
    }

    #[rstest]
    #[case("0 3 6 9 12 15", 18)]
    #[case("1 3 6 10 15 21", 28)]
    #[case("10 13 16 21 30 45", 68)]
    fn test_process_history(#[case] input: &str, #[case] expected: i32) {
        let (_, history) = parse_line(input).expect("should work");
        assert_eq!(expected, process_history(&history));
    }

    #[rstest]
    #[case(
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        "114"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected.to_string(), process(input));
    }
}
