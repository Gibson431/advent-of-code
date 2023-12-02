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
}

pub fn process(input: &str) -> String {
    let num_colors = NumColors {
        red: 12,
        green: 13,
        blue: 14,
    };
    let parts = input.lines();
    let mut sum = 0;
    parts
        .enumerate()
        .filter(|(_i, val)| process_line(val, &num_colors))
        .for_each(|(i, _val)| sum += i + 1);
    sum.to_string()
}

fn process_line(input: &str, num_colors: &NumColors) -> bool {
    let mut isViable = true;
    let mut formatted = input.split(":");
    let formatted = formatted.nth(1);
    formatted
        .unwrap()
        .split(";")
        .map(|val| NumColors::parse(val))
        .for_each(|r| {
            if r.red > num_colors.red || r.green > num_colors.green || r.blue > num_colors.blue {
                isViable = false;
            }
        });
    isViable
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", NumColors { red: 12, green: 13, blue: 14 }, true)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", NumColors { red: 12, green: 13, blue: 14 }, true)]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", NumColors { red: 12, green: 13, blue: 14 }, false)]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", NumColors { red: 12, green: 13, blue: 14 }, false)]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", NumColors { red: 12, green: 13, blue: 14 }, true)]
    fn test_process_line(
        #[case] input: &str,
        #[case] num_colors: NumColors,
        #[case] expected: bool,
    ) {
        assert_eq!(expected, process_line(input, &num_colors));
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            NumColors {
                red: 4,
                green: 0,
                blue: 3
            },
            NumColors::parse(" 3 blue, 4 red")
        );
    }

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8".to_string(), process(input));
    }
}
