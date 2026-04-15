extern crate nom;
use nom::{branch::alt, character::complete::char, combinator::map, IResult, Parser};

fn parse_abc_as_numbers(input: &str) -> IResult<&str, u8> {
    alt((
        map(char('a'), |_| 5),
        map(char('b'), |_| 16),
        map(char('c'), |_| 8),
    )).parse(input)
}

fn main() {
    println!("a: {:?}", parse_abc_as_numbers("a"));
    println!("b: {:?}", parse_abc_as_numbers("b"));
    println!("c: {:?}", parse_abc_as_numbers("c"));
    println!("x: {:?}", parse_abc_as_numbers("x"));
    println!("bjk: {:?}", parse_abc_as_numbers("bjk"));
}
