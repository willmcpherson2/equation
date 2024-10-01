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
    let prog = all_consuming(preceded(junk, many0(terminated(parse_def, junk))))(input);
    match prog {
        Ok((_, prog)) => Ok(prog),
        Err(e) => Err(e.to_string()),
    }
}

fn parse_def(input: &str) -> IResult<&str, Def> {
    let (input, name) = parse_string(input)?;
    let (input, _) = junk(input)?;
    let (input, params) = many0(terminated(parse_string, junk))(input)?;
    let (input, _) = junk(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = junk(input)?;
    let (input, term) = parse_term(input)?;
    let (input, _) = junk(input)?;
    let (input, _) = char(';')(input)?;
    Ok((input, Def { name, params, term }))
}

fn parse_term(input: &str) -> IResult<&str, Term> {
    alt((parse_app, parse_atom))(input)
}

fn parse_app(input: &str) -> IResult<&str, Term> {
    let (input, terms) = many0(terminated(parse_atom, junk))(input)?;
    if terms.len() < 2 {
        return Err(Err::Error(Error::new(input, ErrorKind::Fail)));
    }
    Ok((input, Term::App(terms)))
}

fn parse_atom(input: &str) -> IResult<&str, Term> {
    alt((parse_parens, parse_var))(input)
}

fn parse_parens(input: &str) -> IResult<&str, Term> {
    let (input, _) = char('(')(input)?;
    let (input, _) = junk(input)?;
    let (input, term) = parse_term(input)?;
    let (input, _) = junk(input)?;
    let (input, _) = char(')')(input)?;
    Ok((input, term))
}

fn parse_var(input: &str) -> IResult<&str, Term> {
    map(parse_string, Term::Var)(input)
}

fn parse_string(input: &str) -> IResult<&str, String> {
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
    if def.params.is_empty() {
        format!("{} = {};", def.name, show_term(&def.term))
    } else {
        format!(
            "{} {} = {};",
            def.name,
            def.params.join(" "),
            show_term(&def.term)
        )
    }
}

pub fn show_term(term: &Term) -> String {
    match term {
        Term::Var(var) => var.clone(),
        Term::App(terms) => terms
            .iter()
            .enumerate()
            .map(|(i, term)| match term {
                Term::App(_) if i != 0 => format!("({})", show_term(term)),
                _ => show_term(term),
            })
            .collect::<Vec<_>>()
            .join(" "),
    }
}
