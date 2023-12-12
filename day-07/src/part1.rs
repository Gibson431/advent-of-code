use std::{cmp::Ordering, collections::HashMap};

use nom::{
    bytes::complete::{tag, take_while, take_while1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
struct Hand {
    bet: u32,
    score: u32,
    id: u64,
}

impl Hand {
    fn from_input(hand: &str, bet: &str) -> Hand {
        Hand {
            bet: bet.parse().unwrap(),
            score: parse_cards(hand),
            id: get_card_id(hand.to_string()),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.id == other.id {Ordering::Equal}
        else if self.score == other.score {
            self.id.cmp(&other.id)
        } else {
            self.score.cmp(&other.score)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for Hand {}

fn get_card_id(input: String) -> u64 {
    let mut id: u64 = 0;
    input.chars().for_each(|c| {
        id <<= 4;
        id += match c {
            'A' => 0xE,
            'K' => 0xD,
            'Q' => 0xC,
            'J' => 0xB,
            'T' => 0xA,
            '9' => 0x9,
            '8' => 0x8,
            '7' => 0x7,
            '6' => 0x6,
            '5' => 0x5,
            '4' => 0x4,
            '3' => 0x3,
            '2' => 0x2,
            _ => 0,
        };
    });
    id
}

fn parse_cards(input: &str) -> u32 {
    let mut cards = HashMap::new();
    input.chars().for_each(|c| {
        let count = cards.entry(c).or_insert(0);
        *count += 1;
    });
    let mut weighted_vec: Vec<(&char, &u32)> = cards
        .iter()
        .collect();
    weighted_vec.sort_by(|a, b| a.1.cmp(&b.1).reverse());
    let score = match weighted_vec[0].1 {
        5 => 0x70,
        4 => 0x60,
        3 => {
            if *weighted_vec[1].1 == 2 {
                0x50
            } else {
                0x40
            }
        }
        2 => {
            if *weighted_vec[1].1 == 2 {
                0x30
            } else {
                0x20
            }
        }
        1 => 0x10,
        _ => 0,
    };
    score
}

fn parse_line(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(
        take_while(|c: char| c.is_ascii_alphanumeric()),
        tag(" "),
        take_while1(|c: char| c.is_ascii_alphanumeric()),
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, list) = separated_list1(tag("\n"), parse_line)(input)?;
    Ok((
        input,
        list.iter()
            .map(|val| Hand::from_input(val.0, val.1))
            .collect(),
    ))
}

pub fn process(input: &str) -> String {
    let (_input, mut cards) = parse_input(input).unwrap();
    cards.sort_by(|a, b| a.cmp(&b));
    cards
        .iter()
        .enumerate()
        .map(|(i, val)| val.bet * (i as u32 + 1))
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("32T3K", 0x32A3D)]
    #[case("T55J5", 0xA55B5)]
    #[case("KK677", 0xDD677)]
    #[case("KTJJT", 0xDABBA)]
    #[case("QQQJA", 0xCCCBE)]
    fn test_get_card_id(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(expected, get_card_id(input.to_string()));
    }

    #[rstest]
    #[case("32T3K", 0x20)]
    #[case("T55J5", 0x40)]
    #[case("KK677", 0x30)]
    #[case("KTJJT", 0x30)]
    #[case("QQQJA", 0x40)]
    fn test_parse_cards(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(expected, parse_cards(input));
    }

    #[rstest]
    #[case("32T3K 765", ("32T3K", "765"))]
    #[case("T55J5 684", ("T55J5", "684"))]
    #[case("KK677 28", ("KK677", "28"))]
    #[case("KTJJT 220", ("KTJJT", "220"))]
    #[case("QQQJA 483", ("QQQJA", "483"))]
    fn test_parse_line(#[case] input: &str, #[case] expected: (&str, &str)) {
        assert_eq!(expected, parse_line(input).unwrap().1);
    }

    #[rstest]
    #[case(
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        "6440"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected.to_string(), process(input));
    }
}
