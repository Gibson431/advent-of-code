use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{multispace0, multispace1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use nom_supreme::ParserExt;

fn parse_line(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, id) = take_while1(|c: char| c.is_alphabetic())
        .preceded_by(multispace0)
        .terminated(tag(" = "))
        .parse(input)?;

    let (input, (l, r)) = separated_pair(
        take_while1(|c: char| c.is_alphabetic()),
        tag(", "),
        take_while1(|c: char| c.is_alphabetic()),
    )
    .preceded_by(tag("("))
    .terminated(tag(")"))
    .parse(input)?;

    Ok((input, (id, (l, r))))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<char>, HashMap<&str, (&str, &str)>)> {
    let (input, directions) = take_while1(|c: char| c.is_alphabetic())(input)?;
    let (_input, mappings) = separated_list1(multispace1, parse_line)(input)?;

    let mut hash: HashMap<&str, (&str, &str)> = HashMap::new();
    mappings.iter().for_each(|(i, val)| {
        hash.insert(i, *val);
    });

    Ok((input, (directions.chars().collect::<Vec<char>>(), hash)))
}

pub fn process(input: &str) -> String {
    let (_input, (directions, hash)) = parse_input(input).unwrap();
    let mut current_loc = "AAA";
    let res = directions
        .iter()
        .cycle()
        .position(|&d| {
            current_loc = if d == 'L' {
                hash[current_loc].0
            } else {
                hash[current_loc].1
            };
            current_loc.ends_with("ZZZ")
        })
        .expect("should have ans")
        + 1;

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("AAA = (BBB, CCC)", ("AAA", ("BBB", "CCC")))]
    #[case("\nAAA = (BBB, CCC)", ("AAA", ("BBB", "CCC")))]
    #[case(" AAA = (BBB, CCC)", ("AAA", ("BBB", "CCC")))]
    fn test_process_line(#[case] input: &str, #[case] expected: (&str, (&str, &str))) {
        assert_eq!(expected, parse_line(input).expect("should work").1);
    }

    #[rstest]
    #[case(
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        "2"
    )]
    #[case(
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        "6"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected.to_string(), process(input));
    }
}
