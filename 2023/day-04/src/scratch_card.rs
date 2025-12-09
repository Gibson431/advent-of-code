#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ScratchCard {
    pub winning_numbers: Vec<u32>,
    pub own_numbers: Vec<u32>,
}

impl ScratchCard {
    pub fn new() -> ScratchCard {
        ScratchCard {
            winning_numbers: vec![],
            own_numbers: vec![],
        }
    }

    pub fn parse(input: &str) -> ScratchCard {
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

    pub fn calculate_matches(&self) -> Vec<u32> {
        let mut winners = vec![];
        self.own_numbers.iter().for_each(|s| {
            if self.winning_numbers.contains(s) {
                winners.push(s.clone())
            }
        });
        winners
    }

    pub fn calculate_points(&self) -> u32 {
        let winners = self.calculate_matches();
        let points: u32 = (2 as u32).pow(winners.len() as u32) / 2;

        points
    }
}
