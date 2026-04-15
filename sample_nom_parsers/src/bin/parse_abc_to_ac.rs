extern crate nom;
use nom::{
    character::complete::char,
    IResult,
    Parser,
};

fn parse_abc_to_ac(input: &str) -> IResult<&str, (char, char)> {
    ((char('a'), char('b'), char('c'))).parse(input).map(|(rest, result)| (rest, (result.0, result.2)))
}

fn main() {
    println!("abc: {:?}", parse_abc_to_ac("abc"));
}
