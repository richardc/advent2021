use std::fmt;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    sequence::{delimited, separated_pair},
    IResult,
};

type Number = u32;

#[derive(Debug, PartialEq)]
enum Pair {
    Number(Number),
    Pair(Box<Self>, Box<Self>),
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pair::Number(v) => write!(f, "{}", v),
            Pair::Pair(left, right) => write!(f, "[{},{}]", left, right),
        }
    }
}

#[test]
fn test_pair_display() {
    assert_eq!("2", format!("{}", Pair::Number(2)));
    assert_eq!(
        "[2,3]",
        format!(
            "{}",
            Pair::Pair(Box::new(Pair::Number(2)), Box::new(Pair::Number(3)))
        )
    );

    assert_eq!(
        "[2,[3,4]]",
        format!(
            "{}",
            Pair::Pair(
                Box::new(Pair::Number(2)),
                Box::new(Pair::Pair(
                    Box::new(Pair::Number(3)),
                    Box::new(Pair::Number(4))
                ))
            )
        )
    );
}
fn parse_number(input: &str) -> IResult<&str, Pair> {
    let (input, value) = digit1(input)?;

    Ok((input, Pair::Number(value.parse::<Number>().unwrap())))
}

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    let (input, (left, right)) = delimited(
        char('['),
        separated_pair(
            alt((parse_number, parse_pair)),
            tag(","),
            alt((parse_number, parse_pair)),
        ),
        char(']'),
    )(input)?;

    Ok((input, Pair::Pair(Box::new(left), Box::new(right))))
}

#[test]
fn test_parse_pair() {
    assert_eq!(
        Ok((
            "",
            Pair::Pair(Box::new(Pair::Number(2)), Box::new(Pair::Number(3)))
        )),
        parse_pair("[2,3]")
    );

    assert_eq!(
        Ok((
            "",
            Pair::Pair(
                Box::new(Pair::Number(2)),
                Box::new(Pair::Pair(
                    Box::new(Pair::Number(3)),
                    Box::new(Pair::Number(4))
                ))
            )
        )),
        parse_pair("[2,[3,4]]")
    );
}

fn main() {}
