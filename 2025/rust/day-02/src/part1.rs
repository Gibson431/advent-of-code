use std::ops::RangeInclusive;

use nom::{
    IResult, Parser, bytes::complete::tag,
    character::complete, combinator::all_consuming,
    multi::separated_list1, sequence::separated_pair,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, content) = parse(input).map_err(|e| {
        miette::miette!("failed to parse input, {e}")
    })?;

    dbg!(content);

    Ok("day-02 - part 1".to_string())
}

fn parse(
    input: &str,
) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    let (input, content) = all_consuming(separated_list1(
        tag(","),
        separated_pair(
            complete::u64,
            tag("-"),
            complete::u64,
        ),
    ))
    .parse(input)?;

    let content = content
        .iter()
        .map(|(low, high)| *low..=*high)
        .collect();

    Ok((input, content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!("1227775554", process(input)?);
        Ok(())
    }
}
