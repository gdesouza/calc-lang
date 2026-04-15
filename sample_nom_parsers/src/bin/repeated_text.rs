extern crate nom;
use nom::{
    bytes::complete::take,
    multi::many1,
    Parser,
    IResult,
};

fn repeated_text(input: &str) -> IResult<&str, Vec<&str>> {
    many1(take(3usize)).parse(input)
}

fn main() {
    println!(": {:?}", repeated_text(""));
    println!("abc: {:?}", repeated_text("abc"));
    println!("abcabc: {:?}", repeated_text("abcabc"));
    println!("abcab: {:?}", repeated_text("abcab"));
    println!("ab: {:?}", repeated_text("ab"));
}
