extern crate nom;
use nom::{bytes::complete::tag, IResult, Parser};

fn parse_abc_string(input: &str) -> IResult<&str, &str> {
    tag("abc").parse(input)
}

fn main() {
    println!("abc: {:?}", parse_abc_string("abc"));
    println!("bca: {:?}", parse_abc_string("bca"));
    println!("abcjk: {:?}", parse_abc_string("abcjk"));
}
