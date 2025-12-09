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

impl Pipe {
    fn is_valid_entry(&self, entry: Direction) -> bool {
        match self {
            Pipe::Vertical => match entry {
                Direction::Up => true,
                Direction::Down => true,
                _ => false,
            },
            Pipe::Horizontal => match entry {
                Direction::Left => true,
                Direction::Right => true,
                _ => false,
            },
            Pipe::UpRight => match entry {
                Direction::Down => true,
                Direction::Left => true,
                _ => false,
            },
            Pipe::UpLeft => match entry {
                Direction::Down => true,
                Direction::Right => true,
                _ => false,
            },
            Pipe::DownRight => match entry {
                Direction::Up => true,
                Direction::Left => true,
                _ => false,
            },
            Pipe::DownLeft => match entry {
                Direction::Up => true,
                Direction::Right => true,
                _ => false,
            },
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_ivec(input: IVec2) -> Direction {
        match input {
            IVec2 { x: 0, y: -1 } => Direction::Up,
            IVec2 { x: 0, y: 1 } => Direction::Down,
            IVec2 { x: 1, y: 0 } => Direction::Right,
            IVec2 { x: -1, y: 0 } => Direction::Left,
            _ => panic!(),
        }
    }
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
    let mut current_loc = None;
    let mut angle = IVec2::new(-1, 0);
    while current_loc == None {
        angle = IVec2::new(0, 1).rotate(angle);
        let moved = start.0.to_owned() + angle;
        if map
            .get(&moved)
            .is_some_and(|p| p.is_valid_entry(Direction::from_ivec(angle)))
        {
            current_loc = Some(moved)
        };
    }

    let mut current_loc = current_loc.unwrap();
    let mut last_direction = Direction::from_ivec(angle);
    let mut pipe_length = 1;
    while map.get(&current_loc) != Some(&Pipe::Start) {
        pipe_length += 1;
        match *map.get(&current_loc).unwrap() {
            Pipe::Vertical => {
                if last_direction == Direction::Up {
                    current_loc = current_loc + IVec2::new(0, -1);
                } else {
                    current_loc = current_loc + IVec2::new(0, 1);
                }
            }
            Pipe::Horizontal => {
                if last_direction == Direction::Right {
                    current_loc = current_loc + IVec2::new(1, 0);
                } else {
                    current_loc = current_loc + IVec2::new(-1, 0);
                }
            }
            Pipe::UpRight => {
                if last_direction == Direction::Down {
                    current_loc = current_loc + IVec2::new(1, 0);
                    last_direction = Direction::Right;
                } else {
                    current_loc = current_loc + IVec2::new(0, -1);
                    last_direction = Direction::Up;
                }
            }
            Pipe::UpLeft => {
                if last_direction == Direction::Down {
                    current_loc = current_loc + IVec2::new(-1, 0);
                    last_direction = Direction::Left;
                } else {
                    current_loc = current_loc + IVec2::new(0, -1);
                    last_direction = Direction::Up;
                }
            }
            Pipe::DownRight => {
                if last_direction == Direction::Up {
                    current_loc = current_loc + IVec2::new(1, 0);
                    last_direction = Direction::Right;
                } else {
                    current_loc = current_loc + IVec2::new(0, 1);
                    last_direction = Direction::Down;
                }
            }
            Pipe::DownLeft => {
                if last_direction == Direction::Up {
                    current_loc = current_loc + IVec2::new(-1, 0);
                    last_direction = Direction::Left;
                } else {
                    current_loc = current_loc + IVec2::new(0, 1);
                    last_direction = Direction::Down;
                }
            }
            _ => panic!(),
        };
    }
    dbg!(pipe_length);
    (pipe_length / 2).to_string()
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
    #[case(
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        "8"
    )]
    #[case(
        "-L|F7
7S-7|
L|7||
-L-J|
L|-JF",
        "4"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected.to_string(), process(input));
    }
}
