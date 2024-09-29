use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::{alphanumeric1, char, multispace1},
    combinator::{all_consuming, map, value},
    error::{Error, ErrorKind},
    multi::many0,
    sequence::{pair, preceded, terminated, tuple},
    Err, IResult,
};

pub type Program = Vec<Def>;

#[derive(Debug, Clone)]
pub struct Def {
    pub name: String,
    pub params: Vec<String>,
    pub term: Term,
}

#[derive(Debug, Clone)]
pub enum Term {
    Var(String),
    App(Vec<Term>),
}

pub fn parse_program(input: &str) -> Result<Program, String> {
    let prog = all_consuming(terminated(many0(preceded(junk, parse_def)), junk))(input);
    match prog {
        Ok((_, prog)) => Ok(prog),
        Err(e) => Err(e.to_string()),
    }
}

fn parse_def(input: &str) -> IResult<&str, Def> {
    let (input, name) = parse_var(input)?;
    let (input, _) = junk(input)?;
    let (input, params) = parse_params(input)?;
    let (input, _) = junk(input)?;
    let (input, term) = parse_term(input)?;
    Ok((input, Def { name, params, term }))
}

fn parse_params(input: &str) -> IResult<&str, Vec<String>> {
    let (input, _) = char('(')(input)?;
    let (input, params) = many0(preceded(junk, parse_var))(input)?;
    let (input, _) = junk(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, params))
}

fn parse_term(input: &str) -> IResult<&str, Term> {
    alt((map(parse_app, Term::App), map(parse_var, Term::Var)))(input)
}

fn parse_app(input: &str) -> IResult<&str, Vec<Term>> {
    let (input, _) = char('(')(input)?;
    let (input, terms) = many0(preceded(junk, parse_term))(input)?;
    if terms.len() < 2 {
        return Err(Err::Error(Error::new(input, ErrorKind::Fail)));
    }
    let (input, _) = junk(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, terms))
}

fn parse_var(input: &str) -> IResult<&str, String> {
    map(alphanumeric1, String::from)(input)
}

fn junk(input: &str) -> IResult<&str, ()> {
    value(
        (),
        many0(alt((whitespace, line_comment, multi_line_comment))),
    )(input)
}

fn whitespace(input: &str) -> IResult<&str, ()> {
    value((), multispace1)(input)
}

fn line_comment(input: &str) -> IResult<&str, ()> {
    value((), pair(tag("--"), take_while(|c| c != '\n')))(input)
}

fn multi_line_comment(input: &str) -> IResult<&str, ()> {
    value((), tuple((tag("/*"), take_until("*/"), tag("*/"))))(input)
}

pub fn show_program(prog: &Program) -> String {
    prog.iter().map(show_def).collect::<Vec<_>>().join("\n")
}

fn show_def(def: &Def) -> String {
    format!(
        "{} ({}) {}",
        def.name,
        def.params.join(" "),
        show_term(&def.term)
    )
}

pub fn show_term(term: &Term) -> String {
    match term {
        Term::Var(var) => var.clone(),
        Term::App(terms) => format!(
            "({})",
            terms.iter().map(show_term).collect::<Vec<_>>().join(" ")
        ),
    }
}
