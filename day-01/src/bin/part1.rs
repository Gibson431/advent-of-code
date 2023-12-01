use std::str;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let parts = input.lines();
    let mut sum = 0;
    let numbers: Vec<i32> = parts.map(|part| compute_pair(part.to_string())).collect();
    numbers.into_iter().for_each(|num| sum += num);
    sum.to_string()
}

fn compute_pair(input: String) -> i32 {
    let mut numbers: Vec<i32> = input
        .as_str()
        .chars()
        .filter_map(|c| c.to_string().parse().ok())
        .collect();
    let mut number_string = numbers.first().unwrap_or(&0).clone().to_string();
    number_string.push_str(numbers.pop().unwrap_or(0).to_string().as_str());
    number_string.parse().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let result = part1(input);
        assert_eq!(result, "142".to_string());
    }

    #[test]
    fn test_line() {
        assert_eq!(compute_pair("1abc2".to_string()), 12);
        assert_eq!(compute_pair("pqr3stu8vwx".to_string()), 38);
        assert_eq!(compute_pair("a1b2c3d4e5f".to_string()), 15);
        assert_eq!(compute_pair("treb7uchet".to_string()), 77);
    }
}
