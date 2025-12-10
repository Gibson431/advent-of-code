use itertools::Itertools;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let banks = input.lines();

    let jolts =
        banks.fold(0, |acc, bank| acc + process_bank(bank));

    Ok(jolts.to_string())
}

fn process_bank(bank: &str) -> u64 {
    let bank_as_num: Vec<_> = bank
        .chars()
        .map(|v| v.to_digit(10).unwrap() as u64)
        .collect();

    let mut last_idx = 0;
    let mut acc = 0;
    for i in (0..12).rev() {
        let (idx, jolt) = max_in_iter(
            bank_as_num[last_idx..bank_as_num.len() - i]
                .iter(),
        );
        last_idx = idx;
        acc += jolt * i as u64;
    }

    acc
}

fn max_in_iter<'a>(
    mut iter: impl Iterator<Item = &'a u64> + Clone,
) -> (usize, u64) {
    let num = iter.clone().max().unwrap();
    let idx = iter.position(|x| x == num).unwrap();

    (idx, num.clone())
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
        assert_eq!("3121910778619", process(input)?);
        Ok(())
    }

    #[rstest::rstest]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    fn process_bank_test(
        #[case] input: &str,
        #[case] expected: u64,
    ) {
        assert_eq!(expected, process_bank(input));
    }
}
