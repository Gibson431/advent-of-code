use std::iter::zip;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    multi::separated_list1,
    IResult, Parser,
};

use nom_supreme::ParserExt;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn new(time: u64, record: u64) -> Race {
        Race { time, record }
    }
}

fn parse_input(input: &str) -> IResult<&str, Race> {
    let mut lines = input.lines();
    let (_, times) = tag("Time:")
        .precedes(space1)
        .precedes(separated_list1(space1, digit1))
        .parse(lines.next().unwrap())?;

    let (_input, distances) = tag("Distance:")
        .precedes(space1)
        .precedes(separated_list1(space1, digit1))
        .parse(lines.next().unwrap())?;

    let races = zip(times, distances);
    let races = races.fold((String::new(), String::new()), |mut acc, item| {
        acc.0.push_str(item.0);
        acc.1.push_str(item.1);
        acc
    });

    let race = Race::new(races.0.parse().unwrap(), races.1.parse().unwrap());
    Ok((input, race))
}

pub fn process(input: &str) -> String {
    let (_, race) = parse_input(input).unwrap();
    dbg!(&race);

    let count = (0..race.time)
        .filter_map(|t| (t * (race.time - t) > race.record).then_some(t))
        .count();
    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Time:      7  15   30\nDistance:  9  40  200", "71503")]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected.to_string(), process(input));
    }
}
