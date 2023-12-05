use std::{collections::HashMap, string::ParseError};

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, space0, space1},
    error::{Error, ErrorKind},
    multi::{fold_many1, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
enum Path {
    #[default]
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
    End,
    Error(String),
}

impl Path {
    fn from_str(input: &str) -> Path {
        match input {
            "seed" => Path::Seed,
            "soil" => Path::Soil,
            "fertilizer" => Path::Fertilizer,
            "water" => Path::Water,
            "light" => Path::Light,
            "temperature" => Path::Temperature,
            "humidity" => Path::Humidity,
            "location" => Path::Location,
            "end" => Path::End,
            _ => Path::Error(input.to_string()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
struct SeedMap {
    from: Path,
    to: Path,
    pairs: HashMap<u32, u32>,
}

impl SeedMap {
    fn new(from: Path, to: Path) -> SeedMap {
        SeedMap {
            from,
            to,
            pairs: HashMap::new(),
        }
    }

    fn from_section(input: Vec<&str>) -> SeedMap {
        let mut section = input.iter();
        let (_, (from, to)) = separated_pair(
            alpha1,
            tag::<&str, &str, nom::error::Error<&str>>("-to-"),
            alpha1,
        )(*(section.next().unwrap()))
        .unwrap();
        let from_path = Path::from_str(from);
        let to_path = Path::from_str(to);

        let mut seed_map = SeedMap::new(from_path, to_path);
        section.for_each(|line| {
            let (_, split): (&str, Vec<u32>) =
                separated_list1(space1::<&str, Error<&str>>, complete::u32)(*line).unwrap();
            let mut split = split.iter();
            let dest = *split.next().unwrap();
            let source = *split.next().unwrap();
            let range = *split.last().unwrap();
            for i in 0..range {
                seed_map.pairs.insert(source + i, dest + i);
            }
        });

        seed_map
    }
}

fn input_seeds(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(
        tag("seeds: "),
        fold_many1(
            terminated(complete::u32, space0),
            Vec::new,
            |mut acc: Vec<u32>, item| {
                acc.push(item);
                acc
            },
        ),
    )(input)
}

pub fn process(input: &str) -> String {
    let lines = input.lines().map(|l| l.trim()).collect::<Vec<&str>>();
    let mut sections = lines.split(|l| l.is_empty());
    let (_, starting_points) = input_seeds(sections.next().unwrap().first().unwrap()).unwrap();

    let mut seed_maps: Vec<SeedMap> = sections
        .map(|s| SeedMap::from_section(s.iter().map(|val| *val).collect::<Vec<&str>>()))
        .collect();
    seed_maps.push(SeedMap::new(Path::Location, Path::End));

    let seed_maps = seed_maps.iter();
    let mut locations = vec![];
    starting_points.iter().for_each(|point| {
        let mut num = point;
        let mut current_map = seed_maps
            .clone()
            .find(|val| val.from == Path::Seed)
            .unwrap();
        while current_map.from != Path::Location {
            num = current_map.pairs.get(num).unwrap_or(num);
            current_map = seed_maps
                .clone()
                .find(|map| map.from == current_map.to)
                .unwrap();
        }
        locations.push(num)
    });

    dbg!(locations.clone());

    locations.iter().min().unwrap_or(&&0).to_string()

    // todo!("part 1");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "seeds: 79 14 55 13
        
        seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4",
        "35"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected.to_string(), process(input));
    }
}
