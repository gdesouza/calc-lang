extern crate nom;
use nom::{character::complete::char, Parser, IResult};

fn parse_abc_sequence(input: &str) -> IResult<&str, (char, char, char)> {
    (char('a'), char('b'), char('c')).parse(input)
}

fn main() {
    println!("abc: {:?}", parse_abc_sequence("abc"));
    println!("bca: {:?}", parse_abc_sequence("bca"));
    println!("abcjk: {:?}", parse_abc_sequence("abcjk"));
}
