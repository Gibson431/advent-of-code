use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let banks = input.lines();

    let jolts =
        banks.fold(0, |acc, bank| acc + process_bank(bank));

    Ok(jolts.to_string())
}

fn process_bank(bank: &str) -> u32 {
    let bank_as_num: Vec<_> = bank
        .chars()
        .map(|v| v.to_digit(10).unwrap())
        .collect();

    let first_chunk =
        bank_as_num[0..(bank.len() - 1)].iter();
    let first = first_chunk.clone().max().unwrap();
    let (idx, first) = first_chunk
        .enumerate()
        .find(|(_, x)| *x == first)
        .unwrap();

    let second = bank_as_num[idx + 1..bank.len()]
        .iter()
        .sorted()
        .rev()
        .next()
        .unwrap();

    first * 10 + second
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!("357", process(input)?);
        Ok(())
    }

    #[rstest::rstest]
    #[case("12333345", 45)]
    #[case("989879612", 99)]
    fn process_bank_test(
        #[case] input: &str,
        #[case] expected: u32,
    ) {
        assert_eq!(expected, process_bank(input));
    }
}
