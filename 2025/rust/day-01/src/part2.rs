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
        let mut new_acc = match dir {
            'L' => acc - (amt % 100),
            'R' => acc + (amt % 100),
            _ => panic!("should never get here"),
        };

        count += (amt / 100).abs();

        if new_acc == 0 {
            count += 1;
        }

        if new_acc != new_acc % 100 {
            new_acc %= 100;
            count += 1;
        }

        if new_acc < 0 {
            new_acc += 100;
            if acc != 0 {
                count += 1;
            }
        }

        new_acc
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
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
