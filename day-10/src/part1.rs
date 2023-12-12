use std::collections::HashMap;

use glam::IVec2;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Pipe {
    Start,
    Vertical,
    Horizontal,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
    NULL,
}

fn parse_input(input: &str) -> HashMap<IVec2, Pipe> {
    let mut map = HashMap::new();
    let lines = input.lines().enumerate();
    lines.for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| {
            let pipe = match c {
                'S' => Pipe::Start,
                '|' => Pipe::Vertical,
                '-' => Pipe::Horizontal,
                'L' => Pipe::UpRight,
                'J' => Pipe::UpLeft,
                'F' => Pipe::DownRight,
                '7' => Pipe::DownLeft,
                _ => Pipe::NULL,
            };

            map.insert(IVec2::new(x as i32, y as i32), pipe);
        })
    });
    map
}

pub fn process(input: &str) -> String {
    let map = parse_input(input);
    let start = map
        .iter()
        .find_map(|f| (*f.1 == Pipe::Start).then_some(f))
        .expect("should exist");
    let current_loc = start.0;
    let mut pipe_length = 0;
    while map.get(current_loc) != Some(&Pipe::Start) {
        pipe_length += 1;
        todo!("process")
    }
    todo!("part 1")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        ".....
    .S-7.
    .|.|.
    .L-J.
    .....",
        "4"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected.to_string(), process(input));
    }
}
