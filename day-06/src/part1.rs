use std::iter::zip;

use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_till, take_until, take_while1},
    character::{
        complete::{self, multispace0, multispace1, newline, space0, space1},
        is_digit, is_space,
        streaming::alpha1,
    },
    error::ParseError,
    multi::{fold_many0, fold_many1, separated_list0, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult, Parser,
};

use nom_supreme::ParserExt;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Race {
    time: u32,
    record: u32,
}

impl Race {
    fn new(time: u32, record: u32) -> Race {
        Race { time, record }
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Race>> {
    let mut lines = input.lines();
    let (input, times) = tag("Time:")
        .precedes(space1)
        .precedes(separated_list1(space1, complete::u32))
        .parse(lines.next().unwrap())?;

    let (input, distances) = tag("Distance:")
        .precedes(space1)
        .precedes(separated_list1(space1, complete::u32))
        .parse(lines.next().unwrap())?;

    let races = zip(times, distances);
    let races = races.map(|tup| Race::new(tup.0, tup.1)).collect();

    Ok((input, races))
}

pub fn process(input: &str) -> String {
    let races = parse_input(input);

    dbg!(&races);
    todo!("part 1")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Time:      7  15   30\nDistance:  9  40  200", "288")]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected.to_string(), process(input));
    }
}
