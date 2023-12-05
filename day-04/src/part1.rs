#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
struct ScratchCard {
    winning_numbers: Vec<u32>,
    own_numbers: Vec<u32>,
}

impl ScratchCard {
    fn new() -> ScratchCard {
        ScratchCard {
            winning_numbers: vec![],
            own_numbers: vec![],
        }
    }

    fn parse(input: &str) -> ScratchCard {
        let mut new_card = ScratchCard::new();
        let vecs = input
            .split(":")
            .nth(1)
            .expect("wrong format")
            .split("|")
            .map(|s| {
                s.trim()
                    .replace("  ", " ")
                    .split(" ")
                    .map(|val| val.parse().unwrap_or(0))
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        new_card.winning_numbers = vecs.first().expect("vec should exist").to_owned();
        new_card.own_numbers = vecs.last().expect("vec should exist").to_owned();

        new_card
    }
}

fn process_card(input: &str) -> String {
    let new_card = ScratchCard::parse(input);
    let mut winners = vec![];
    new_card.own_numbers.iter().for_each(|s| {
        if new_card.winning_numbers.contains(s) {
            winners.push(s)
        }
    });
    let points: u32 = (2 as u32).pow(winners.len() as u32) / 2;
    points.to_string()
}

pub fn process(input: &str) -> String {
    let lines = input.lines();
    let points = lines.map(|l| process_card(l).parse::<u32>().expect("wtf"));
    let sum: u32 = points.sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", "8")]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", "2")]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", "2")]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", "1")]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", "0")]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", "0")]
    fn test_process_card(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected.to_string(), process_card(input));
    }

    #[rstest]
    #[case(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 
        ScratchCard{winning_numbers: vec![41, 48, 83, 86, 17], own_numbers: vec![83, 86,  6, 31, 17,  9, 48, 53]}
    )]
    #[case(
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 
        ScratchCard{winning_numbers: vec![13, 32, 20, 16, 61], own_numbers: vec![61, 30, 68, 82, 17, 32, 24, 19]}
    )]
    #[case(
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 
        ScratchCard{winning_numbers: vec![ 1, 21, 53, 59, 44], own_numbers: vec![69, 82, 63, 72, 16, 21, 14,  1]}
    )]
    #[case(
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 
        ScratchCard{winning_numbers: vec![41, 92, 73, 84, 69], own_numbers: vec![59, 84, 76, 51, 58,  5, 54, 83]}
    )]
    #[case(
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 
        ScratchCard{winning_numbers: vec![87, 83, 26, 28, 32], own_numbers: vec![88, 30, 70, 12, 93, 22, 82, 36]}
    )]
    #[case(
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 
        ScratchCard{winning_numbers: vec![31, 18, 13, 56, 72], own_numbers: vec![74, 77, 10, 23, 35, 67, 36, 11]}
    )]
    fn test_parse(#[case] input: &str, #[case] expected: ScratchCard) {
        assert_eq!(expected, ScratchCard::parse(input));
    }

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", "13")]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected.to_string(), process(input));
    }
}
