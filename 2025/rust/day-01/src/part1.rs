use miette::miette;
use nom::{
    IResult, Parser,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::pair,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_input, commands) = parse(input).map_err(|e| {
        miette!("failed to parse input, {e}")
    })?;

    let mut count = 0;
    commands.iter().fold(50, |acc: i32, (dir, amt)| {
        let mut acc = match dir {
            'L' => acc - amt,
            'R' => acc + amt,
            _ => panic!("should never get here"),
        };
        acc %= 100;

        if acc == 0 {
            count += 1;
        }

        acc
    });
    Ok(count.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<(char, i32)>> {
    separated_list1(
        line_ending,
        pair(complete::one_of("LR"), complete::i32),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
