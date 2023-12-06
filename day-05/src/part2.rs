use std::ops::Range;

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, space0, space1},
    error::Error,
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
    pairs: Vec<(Range<u64>, Range<u64>)>,
}

impl SeedMap {
    fn new(from: Path, to: Path) -> SeedMap {
        SeedMap {
            from,
            to,
            pairs: Vec::new(),
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
            let (_, split): (&str, Vec<u64>) =
                separated_list1(space1::<&str, Error<&str>>, complete::u64)(*line).unwrap();
            let mut split = split.iter();
            let dest = *split.next().unwrap();
            let source = *split.next().unwrap();
            let range = *split.last().unwrap();
            seed_map
                .pairs
                .push((source..source + range, dest..dest + range));
        });

        seed_map
    }
}

fn input_seeds(input: &str) -> IResult<&str, Vec<Range<u64>>> {
    preceded(
        tag("seeds: "),
        fold_many1(
            separated_pair(complete::u64, space0, complete::u64),
            Vec::new,
            |mut acc: Vec<Range<u64>>, (start, range)| {
                acc.push(start..start + range);
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
        let mut num = *point;
        let mut current_map = seed_maps
            .clone()
            .find(|val| val.from == Path::Seed)
            .unwrap();
        while current_map.from != Path::Location {
            let mut temp_num = num;
            current_map
                .pairs
                .clone()
                .into_iter()
                .for_each(|(from, to)| {
                    if from.contains(&num) {
                        temp_num = (num - from.start) + to.start;
                    }
                });
            num = temp_num.clone();
            current_map = seed_maps
                .clone()
                .find(|map| map.from == current_map.to)
                .unwrap();
        }
        locations.push(num)
    });

    locations.iter().min().unwrap_or(&&0).to_string()
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
