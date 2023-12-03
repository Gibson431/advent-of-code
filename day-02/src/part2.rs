#[derive(Clone, Copy, PartialEq, Debug)]
struct NumColors {
    red: u32,
    green: u32,
    blue: u32,
}

impl NumColors {
    fn new() -> NumColors {
        NumColors {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    fn parse(input: &str) -> NumColors {
        let mut num_colors = NumColors::new();
        let input = input.trim().replace(",", "");
        let split: Vec<&str> = input.split(" ").collect();
        for i in 0..split.len() {
            if i % 2 == 0 && i + 1 < split.len() {
                let color = split.iter().nth(i + 1).unwrap();
                if color == &"red" {
                    num_colors.red = split.iter().nth(i).unwrap().parse::<u32>().unwrap_or(0);
                } else if color == &"green" {
                    num_colors.green = split.iter().nth(i).unwrap().parse::<u32>().unwrap_or(0);
                } else if color == &"blue" {
                    num_colors.blue = split.iter().nth(i).unwrap().parse::<u32>().unwrap_or(0);
                }
            };
        }

        num_colors
    }
    fn max(&mut self, input: NumColors) {
        self.red = std::cmp::max(self.red, input.red);
        self.green = std::cmp::max(self.green, input.green);
        self.blue = std::cmp::max(self.blue, input.blue);
    }
}

pub fn process(input: &str) -> String {
    let parts = input.lines();
    let maxes = parts.map(|line| process_line(line));
    let mut sum = 0;
    maxes.for_each(|s| sum += s.parse::<i32>().unwrap_or(0));
    sum.to_string()
}

fn process_line(input: &str) -> String {
    let mut current_colors = NumColors::new();
    let formatted = input.split(":").nth(1).unwrap();
    let rounds = formatted.split(";").map(|val| NumColors::parse(val));
    rounds.for_each(|r| current_colors.max(r));
    (current_colors.red * current_colors.green * current_colors.blue).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", "48")]
    #[case(
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "12"
    )]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "1560"
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "630"
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", "36")]
    fn test_process_line(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected.to_string(), process_line(input));
    }

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286".to_string(), process(input));
    }
}
