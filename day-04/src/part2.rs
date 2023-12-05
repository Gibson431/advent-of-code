use crate::scratch_card::*;

pub fn process(input: &str) -> String {
    let cards: Vec<ScratchCard> = input.lines().map(|l| ScratchCard::parse(l)).collect();
    let mut num_cards = vec![1; cards.len()];
    let num_lines = num_cards.len();

    cards.clone().iter().enumerate().for_each(|(index, card)| {
        for _ in 0..num_cards[index] {
            let matches = card.calculate_matches().len();
            for p in 0..matches {
                if index + 1 + p as usize >= num_lines {
                    num_cards[num_lines - 1] += 1;
                } else {
                    num_cards[index + 1 + p as usize] += 1;
                }
            }
        }
    });

    let total_cards: u32 = num_cards.iter().sum();
    total_cards.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        "30"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected.to_string(), process(input));
    }
}
