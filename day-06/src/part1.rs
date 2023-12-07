use std::iter::zip;

use nom::{
    bytes::complete::tag,
    character::complete::{self, space1},
    multi::separated_list1,
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
    let (_, times) = tag("Time:")
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
    let (_, races) = parse_input(input).unwrap();
    dbg!(&races);

    let counts = races.iter().map(|r| {
        (0..r.time)
            .filter_map(|t| (t * (r.time - t) > r.record).then_some(t))
            .count()
    });
    counts.product::<usize>().to_string()
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
