extern crate nom;
use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take_while,
    character::complete::{alpha1, char},
    combinator::map,
    multi::many0,
    number::complete::double,
    sequence::{delimited, preceded},
    Parser,
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum ParsedFactor<'a> {
    Literal(f64),
    Identifier(&'a str),
    SubExpression(Box<ParsedExpr<'a>>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TermOperator {
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ExprOperator {
    Add,
    Subtract,
}

pub type ParsedTerm<'a> = (ParsedFactor<'a>, Vec<(TermOperator, ParsedFactor<'a>)>);

pub type ParsedExpr<'a> = (ParsedTerm<'a>, Vec<(ExprOperator, ParsedTerm<'a>)>);

#[derive(Debug, PartialEq)]
pub enum ParsedStatement<'a> {
    Declaration(&'a str),
    InputOperation(&'a str),
    OutputOperation(ParsedExpr<'a>),
    Assignment(&'a str, ParsedExpr<'a>),
}

pub type ParsedProgram<'a> = Vec<ParsedStatement<'a>>;

pub fn parse_program(input: &str) -> IResult<&str, ParsedProgram<'_>> {
    many0(preceded(
        skip_spaces,
        alt((
            parse_declaration,
            parse_input_statement,
            parse_output_statement,
            parse_assignment,
        )),
    )).parse(input)
}

fn parse_declaration(input: &str) -> IResult<&str, ParsedStatement<'_>> {
    (char('@'), skip_spaces, parse_identifier).parse(input)
        .map(|(input, output)| (input, ParsedStatement::Declaration(output.2)))
}

fn parse_input_statement(input: &str) -> IResult<&str, ParsedStatement<'_>> {
    (char('>'), skip_spaces, parse_identifier).parse(input)
        .map(|(input, output)| (input, ParsedStatement::InputOperation(output.2)))
}

fn parse_output_statement(input: &str) -> IResult<&str, ParsedStatement<'_>> {
    (char('<'), skip_spaces, parse_expr).parse(input)
        .map(|(input, output)| (input, ParsedStatement::OutputOperation(output.2)))
}

fn parse_assignment(input:&str) -> IResult<&str, ParsedStatement<'_>> {
    ( parse_identifier, 
      skip_spaces, 
      tag(":="),
      skip_spaces,
      parse_expr,
    ).parse(input)
    .map(|(input, output)| (input, ParsedStatement::Assignment(output.0, output.4)))
}

fn parse_identifier(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

fn parse_subexpr(input: &str) -> IResult<&str, ParsedExpr<'_>> {
    delimited(
        preceded(skip_spaces, char('(')),
        parse_expr,
        preceded(skip_spaces, char(')')),
    ).parse(input)
}

fn parse_factor(input: &str) -> IResult<&str, ParsedFactor<'_>> {
    preceded(
        skip_spaces,
        alt((
            map(parse_identifier, ParsedFactor::Identifier),
            map(double, ParsedFactor::Literal),
            map(parse_subexpr, |expr| {
                ParsedFactor::SubExpression(Box::new(expr))
            }),
        )),
    ).parse(input)
}

fn parse_term(input:&str) -> IResult<&str, ParsedTerm<'_>> {
    (
        parse_factor,
        many0((
            preceded(
                skip_spaces,
                alt((
                    map(char('*'), |_| TermOperator::Multiply),
                    map(char('/'), |_| TermOperator::Divide)
                )),
            ),
            parse_factor,
        )),
    ).parse(input)
}

fn parse_expr(input: &str) -> IResult<&str, ParsedExpr<'_>> {
    (
        parse_term,
        many0((
            preceded(
                skip_spaces,
                alt((
                    map(char('+'), |_| ExprOperator::Add),
                    map(char('-'), |_| ExprOperator::Subtract)
                )),
            ),
            parse_term,
        )),
    ).parse(input)
}

fn skip_spaces(input: &str) -> IResult<&str, &str> {
    let chars = " \t\r\n";
    take_while(move |ch| chars.contains(ch)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lit(v: f64) -> ParsedFactor<'static> {
        ParsedFactor::Literal(v)
    }

    fn ident(s: &str) -> ParsedFactor<'_> {
        ParsedFactor::Identifier(s)
    }

    fn simple_term(f: ParsedFactor<'_>) -> ParsedTerm<'_> {
        (f, vec![])
    }

    fn simple_expr(f: ParsedFactor<'_>) -> ParsedExpr<'_> {
        (simple_term(f), vec![])
    }

    // identifier

    #[test]
    fn test_parse_identifier() {
        assert_eq!(parse_identifier("abc"), Ok(("", "abc")));
    }

    #[test]
    fn test_parse_identifier_stops_at_digit() {
        assert_eq!(parse_identifier("xy3z"), Ok(("3z", "xy")));
    }

    #[test]
    fn test_parse_identifier_fails_on_digit_start() {
        assert!(parse_identifier("123").is_err());
    }

    // factor

    #[test]
    fn test_parse_factor_literal() {
        let (rest, factor) = parse_factor("42").unwrap();
        assert_eq!(rest, "");
        assert_eq!(factor, lit(42.0));
    }

    #[test]
    fn test_parse_factor_identifier() {
        let (rest, factor) = parse_factor("abc").unwrap();
        assert_eq!(rest, "");
        assert_eq!(factor, ident("abc"));
    }

    #[test]
    fn test_parse_factor_subexpression() {
        let (rest, factor) = parse_factor("(1+2)").unwrap();
        assert_eq!(rest, "");
        let expected = ParsedFactor::SubExpression(Box::new((
            simple_term(lit(1.0)),
            vec![(ExprOperator::Add, simple_term(lit(2.0)))],
        )));
        assert_eq!(factor, expected);
    }

    // term

    #[test]
    fn test_parse_term_single_factor() {
        let (rest, term) = parse_term("7").unwrap();
        assert_eq!(rest, "");
        assert_eq!(term, simple_term(lit(7.0)));
    }

    #[test]
    fn test_parse_term_multiply() {
        let (rest, term) = parse_term("3*4").unwrap();
        assert_eq!(rest, "");
        assert_eq!(term, (lit(3.0), vec![(TermOperator::Multiply, lit(4.0))]));
    }

    #[test]
    fn test_parse_term_divide() {
        let (rest, term) = parse_term("10/2").unwrap();
        assert_eq!(rest, "");
        assert_eq!(term, (lit(10.0), vec![(TermOperator::Divide, lit(2.0))]));
    }

    #[test]
    fn test_parse_term_chained() {
        let (rest, term) = parse_term("2*3/4").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            term,
            (
                lit(2.0),
                vec![
                    (TermOperator::Multiply, lit(3.0)),
                    (TermOperator::Divide, lit(4.0)),
                ]
            )
        );
    }

    // expr

    #[test]
    fn test_parse_expr_single_term() {
        let (rest, expr) = parse_expr("5").unwrap();
        assert_eq!(rest, "");
        assert_eq!(expr, simple_expr(lit(5.0)));
    }

    #[test]
    fn test_parse_expr_add() {
        let (rest, expr) = parse_expr("1+2").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            expr,
            (simple_term(lit(1.0)), vec![(ExprOperator::Add, simple_term(lit(2.0)))])
        );
    }

    #[test]
    fn test_parse_expr_subtract() {
        let (rest, expr) = parse_expr("5-3").unwrap();
        assert_eq!(rest, "");
        assert_eq!(
            expr,
            (simple_term(lit(5.0)), vec![(ExprOperator::Subtract, simple_term(lit(3.0)))])
        );
    }

    #[test]
    fn test_parse_expr_mixed_precedence() {
        let (rest, expr) = parse_expr("1+2*3").unwrap();
        assert_eq!(rest, "");
        let term2 = (lit(2.0), vec![(TermOperator::Multiply, lit(3.0))]);
        assert_eq!(
            expr,
            (simple_term(lit(1.0)), vec![(ExprOperator::Add, term2)])
        );
    }

    // statements

    #[test]
    fn test_parse_declaration() {
        let (rest, stmt) = parse_declaration("@foo").unwrap();
        assert_eq!(rest, "");
        assert_eq!(stmt, ParsedStatement::Declaration("foo"));
    }

    #[test]
    fn test_parse_input_statement() {
        let (rest, stmt) = parse_input_statement(">bar").unwrap();
        assert_eq!(rest, "");
        assert_eq!(stmt, ParsedStatement::InputOperation("bar"));
    }

    #[test]
    fn test_parse_output_statement() {
        let (rest, stmt) = parse_output_statement("<42").unwrap();
        assert_eq!(rest, "");
        assert_eq!(stmt, ParsedStatement::OutputOperation(simple_expr(lit(42.0))));
    }

    #[test]
    fn test_parse_assignment() {
        let (rest, stmt) = parse_assignment("x:=10").unwrap();
        assert_eq!(rest, "");
        assert_eq!(stmt, ParsedStatement::Assignment("x", simple_expr(lit(10.0))));
    }

    #[test]
    fn test_parse_assignment_with_spaces() {
        let (rest, stmt) = parse_assignment("x := 10").unwrap();
        assert_eq!(rest, "");
        assert_eq!(stmt, ParsedStatement::Assignment("x", simple_expr(lit(10.0))));
    }

    // program

    #[test]
    fn test_parse_empty_program() {
        let (rest, program) = parse_program("").unwrap();
        assert_eq!(rest, "");
        assert!(program.is_empty());
    }

    #[test]
    fn test_parse_program_single_statement() {
        let (rest, program) = parse_program("@x").unwrap();
        assert_eq!(rest, "");
        assert_eq!(program.len(), 1);
        assert_eq!(program[0], ParsedStatement::Declaration("x"));
    }

    #[test]
    fn test_parse_program_multiple_statements() {
        let (rest, program) = parse_program("@a\n>a\n<a+1").unwrap();
        assert_eq!(rest, "");
        assert_eq!(program.len(), 3);
        assert_eq!(program[0], ParsedStatement::Declaration("a"));
        assert_eq!(program[1], ParsedStatement::InputOperation("a"));
        assert_eq!(
            program[2],
            ParsedStatement::OutputOperation((
                simple_term(ident("a")),
                vec![(ExprOperator::Add, simple_term(lit(1.0)))],
            ))
        );
    }

    #[test]
    fn test_parse_sum_example() {
        let input = "@a\n@b\n>a\n>b\n<a+b";
        let (rest, program) = parse_program(input).unwrap();
        assert_eq!(rest, "");
        assert_eq!(program.len(), 5);
        assert_eq!(program[0], ParsedStatement::Declaration("a"));
        assert_eq!(program[1], ParsedStatement::Declaration("b"));
        assert_eq!(program[2], ParsedStatement::InputOperation("a"));
        assert_eq!(program[3], ParsedStatement::InputOperation("b"));
        assert_eq!(
            program[4],
            ParsedStatement::OutputOperation((
                simple_term(ident("a")),
                vec![(ExprOperator::Add, simple_term(ident("b")))],
            ))
        );
    }

    #[test]
    fn test_parse_complex_expression() {
        let (rest, expr) = parse_expr("(a+b)*(c-d)").unwrap();
        assert_eq!(rest, "");
        let sub1 = ParsedFactor::SubExpression(Box::new((
            simple_term(ident("a")),
            vec![(ExprOperator::Add, simple_term(ident("b")))],
        )));
        let sub2 = ParsedFactor::SubExpression(Box::new((
            simple_term(ident("c")),
            vec![(ExprOperator::Subtract, simple_term(ident("d")))],
        )));
        assert_eq!(
            expr,
            ((sub1, vec![(TermOperator::Multiply, sub2)]), vec![])
        );
    }
}
