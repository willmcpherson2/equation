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

pub fn parse_program(text: &str) -> Result<Program, String> {
    let program = all_consuming(terminated(many0(preceded(junk, parse_def)), junk))(text);
    match program {
        Ok((_, program)) => Ok(program),
        Err(e) => Err(e.to_string()),
    }
}

fn parse_def(text: &str) -> IResult<&str, Def> {
    let (text, name) = parse_var(text)?;
    let (text, _) = junk(text)?;
    let (text, params) = parse_params(text)?;
    let (text, _) = junk(text)?;
    let (text, term) = parse_term(text)?;
    Ok((text, Def { name, params, term }))
}

fn parse_params(text: &str) -> IResult<&str, Vec<String>> {
    let (text, _) = char('(')(text)?;
    let (text, params) = many0(preceded(junk, parse_var))(text)?;
    let (text, _) = junk(text)?;
    let (text, _) = char(')')(text)?;
    Ok((text, params))
}

fn parse_term(text: &str) -> IResult<&str, Term> {
    alt((map(parse_app, Term::App), map(parse_var, Term::Var)))(text)
}

fn parse_app(text: &str) -> IResult<&str, Vec<Term>> {
    let (text, _) = char('(')(text)?;
    let (text, terms) = many0(preceded(junk, parse_term))(text)?;
    if terms.len() < 2 {
        return Err(Err::Error(Error::new(text, ErrorKind::Fail)));
    }
    let (text, _) = junk(text)?;
    let (text, _) = char(')')(text)?;
    Ok((text, terms))
}

fn parse_var(text: &str) -> IResult<&str, String> {
    map(alphanumeric1, String::from)(text)
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

pub fn show_program(program: &Program) -> String {
    program.iter().map(show_def).collect::<Vec<_>>().join("\n")
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
