pub fn process(input: &str) -> String {
    let lines = input.lines();
    let mut valid_nums = vec![];
    let symbols = lines.clone().map(|r| extract_symbols(r));
    let numbers = lines.clone().map(|r| extract_numbers(r));
    numbers.enumerate().for_each(|(rowi, row)| {
        row.iter().for_each(|num| {
            let start_index = lines
                .clone()
                .nth(rowi.clone())
                .expect("is index")
                .find(num.to_string().as_str())
                .expect("exists");
            let mut indexes = "".to_string();
            if start_index != 0 {
                indexes = format!("{}", start_index - 1).to_string()
            }
            for i in 0..=num.to_string().len() {
                if start_index + i + 1 < lines.clone().nth(rowi).expect("index").len() {
                    indexes = format!("{}{}", indexes, start_index + i)
                }
            }
            let mut should_add = false;
            if rowi == 0 {
                symbols.clone().enumerate().for_each(|(i, syms)| {
                    if i == rowi || i == rowi + 1 {
                        syms.iter().for_each(|s| {
                            if indexes.contains(s.to_string().as_str()) {
                                should_add = true;
                            };
                        });
                    }
                });
            } else {
                symbols.clone().enumerate().for_each(|(i, syms)| {
                    if i == rowi || i == rowi + 1 || i == rowi - 1 {
                        let _ = syms.iter().for_each(|s| {
                            if indexes.contains(s.to_string().as_str()) {
                                should_add = true;
                            }
                        });
                    }
                });
            }

            if should_add {
                valid_nums.push(num.clone());
            }
        })
    });

    let mut sum = 0;
    dbg!(valid_nums.clone());
    valid_nums.iter().for_each(|val| sum += val);

    sum.to_string()
    // "".to_string()
}
fn extract_symbols(input: &str) -> Vec<usize> {
    let symbols: Vec<usize> = input
        .trim()
        .chars()
        .enumerate()
        .filter(|(_, c)| c != &'.' && !c.is_digit(10) && c != &' ')
        .map(|(i, _)| i)
        .collect();
    symbols
}

fn extract_numbers(input: &str) -> Vec<usize> {
    let numbers = input
        .chars()
        .map(|c| if c.is_numeric() { c } else { ' ' })
        .collect::<String>()
        .split(" ")
        .filter(|val| val.parse::<usize>().unwrap_or(0) != 0)
        .map(|val| val.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
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
    fn test_extract_numbers(#[case] input: &str, #[case] expected: Vec<usize>) {
        assert_eq!(expected, extract_numbers(input));
    }

    #[rstest]
    #[case(
        "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..",
        "4361"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process(input));
    }
}
