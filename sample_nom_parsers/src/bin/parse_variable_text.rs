extern crate nom;
use nom::{
    bytes::complete::take,
    character::complete::char,
    Parser,
    IResult,
};

fn parse_variable_text(input: &str) -> IResult<&str, (char, &str)> {
    ((char('n'), take(2usize))).parse(input)
}

fn main() {
    println!("n: {:?}", parse_variable_text("n"));
    println!("n12: {:?}", parse_variable_text("n12"));
    println!("n123: {:?}", parse_variable_text("n123"));
    println!("x12: {:?}", parse_variable_text("x12"));
    println!("nghj: {:?}", parse_variable_text("nghj"));
    println!("xghj: {:?}", parse_variable_text("xghj"));
    println!("ng: {:?}", parse_variable_text("ng"));
}
