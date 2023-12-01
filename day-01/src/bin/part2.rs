use std::str;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let parts = input.lines();
    let mut sum = 0;
    let numbers: Vec<i32> = parts.map(|part| compute_pair(part.to_string())).collect();
    numbers.into_iter().for_each(|num| sum += num);
    sum.to_string()
}

fn compute_pair(input: String) -> i32 {
    let input = convert_word_to_num(input);
    let mut numbers: Vec<i32> = input
        .as_str()
        .chars()
        .filter_map(|c| c.to_string().parse().ok())
        .collect();
    let mut number_string = numbers.first().unwrap_or(&0).clone().to_string();
    number_string.push_str(numbers.pop().unwrap_or(0).to_string().as_str());
    number_string.parse().unwrap_or(0)
}

fn convert_word_to_num(input: String) -> String {
    input
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let result = part2(input);
        assert_eq!(result, "281".to_string());
    }

    #[test]
    fn test_line() {
        assert_eq!(compute_pair("two1nine".to_string()), 29);
        assert_eq!(compute_pair("eightwothree".to_string()), 83);
        assert_eq!(compute_pair("abcone2threexyz".to_string()), 13);
        assert_eq!(compute_pair("xtwone3four".to_string()), 24);
        assert_eq!(compute_pair("4nineeightseven2".to_string()), 42);
        assert_eq!(compute_pair("zoneight234".to_string()), 14);
        assert_eq!(compute_pair("7pqrstsixteen".to_string()), 76);
    }
}
