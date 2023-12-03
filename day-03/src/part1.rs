use std::vec;

pub fn process(input: &str) -> String {
    let lines = input.lines();
    let symbols: Vec<Vec<usize>> = lines.clone().map(|r| extract_symbols(r)).collect();
    let numbers: Vec<Vec<u32>> = lines.map(|r| extract_numbers(r)).collect();
    "".to_string()
}
fn extract_symbols(input: &str) -> Vec<usize> {
    let symbols: Vec<usize> = input
        .chars()
        .enumerate()
        .filter(|(_, c)| c != &'.' && !c.is_digit(10) && c != &' ')
        .map(|(i, _)| i)
        .collect();
    symbols
}

fn extract_numbers(input: &str) -> Vec<u32> {
    let numbers = input
        .chars()
        .map(|c| if c.is_numeric() { c } else { ' ' })
        .collect::<String>()
        .split(" ")
        .filter(|val| val.parse::<u32>().unwrap_or(0) != 0)
        .map(|val| val.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    numbers
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("467..114..", vec![])]
    #[case("...*......", vec![3])]
    #[case("..35..633.", vec![])]
    #[case("......#...", vec![6])]
    #[case("617*......", vec![3])]
    #[case(".....+.58.", vec![5])]
    #[case("..592.....", vec![])]
    #[case("......755.", vec![])]
    #[case("...$.*....", vec![3,5])]
    #[case(".664.598..", vec![])]
    fn test_symbols(#[case] input: &str, #[case] expected: Vec<usize>) {
        assert_eq!(expected, extract_symbols(input));
    }

    #[rstest]
    #[case("467..114..", vec![467, 114])]
    #[case("...*......", vec![])]
    #[case("..35..633.", vec![35,633])]
    #[case("......#...", vec![])]
    #[case("617*......", vec![617])]
    #[case(".....+.58.", vec![58])]
    #[case("..592.....", vec![592])]
    #[case("......755.", vec![755])]
    #[case("...$.*....", vec![])]
    #[case(".664.598..", vec![664,598])]
    fn test_extract_numbers(#[case] input: &str, #[case] expected: Vec<u32>) {
        assert_eq!(expected, extract_numbers(input));
    }

    #[rstest]
    #[case(
        "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..",
        "4361"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process(input));
    }
}
