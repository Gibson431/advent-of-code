use std::ops::RangeInclusive;

use itertools::Itertools;
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

    let count = content.iter().fold(0, |acc, range| {
        acc + count_range(range.clone())
    });

    Ok(count.to_string())
}

fn count_range(range: RangeInclusive<u64>) -> u64 {
    range.fold(0, |acc, num| {
        let num_str = num.to_string();

        if (0..num_str.len() / 2).any(|i| {
            let chunks = num_str
                .bytes()
                .chunks(i+1);
            chunks.into_iter().all(|c| c.eq(num_str[0..=i].bytes()))
        }) {
            acc + num
        } else {
            acc
        }
    })
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
        assert_eq!("4174379265", process(input)?);
        Ok(())
    }

    #[rstest::rstest]
    #[case(95..=115, 210)]
    #[case(10100..=10102, 0)]
    fn count_range_test(
        #[case] input: RangeInclusive<u64>,
        #[case] expected: u64,
    ) {
        assert_eq!(expected, count_range(input))
    }
}
