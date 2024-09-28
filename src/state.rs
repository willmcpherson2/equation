use std::collections::HashMap;

use crate::{show_term, Def, Program, Term};

#[derive(Debug, Clone)]
pub struct State {
    names: Vec<String>,
    procs: Vec<Procedure>,
    stack: Stack,
    args: Stack,
}

#[derive(Debug, Clone)]
pub struct Procedure {
    arity: usize,
    body: Stack,
}

pub type Stack = Vec<Op>;

#[derive(Debug, Clone, Copy)]
pub enum Op {
    App,
    Def(usize),
    Arg(usize),
}

pub fn compile(program: &Program) -> State {
    let names = program.iter().map(|def| def.name.clone()).collect();

    let def_indices: HashMap<String, usize> = program
        .iter()
        .enumerate()
        .map(|(i, def)| (def.name.clone(), i))
        .collect();
    let procs: Vec<Procedure> = program
        .iter()
        .map(|def| Procedure {
            arity: def.params.len(),
            body: compile_def(def, &def_indices),
        })
        .collect();

    let stack = program
        .iter()
        .position(|def| def.name == "Main")
        .and_then(|index| procs.get(index).cloned())
        .map(|proc| proc.body)
        .unwrap_or(vec![]);

    let args = vec![];

    State {
        names,
        procs,
        stack,
        args,
    }
}

fn compile_def(def: &Def, def_indices: &HashMap<String, usize>) -> Stack {
    let param_indices: HashMap<String, usize> = def
        .params
        .iter()
        .enumerate()
        .map(|(i, param)| (param.clone(), i))
        .collect();
    let mut stack = compile_term(&def.term, def_indices, &param_indices);
    stack.reverse();
    stack
}

fn compile_term(
    term: &Term,
    def_indices: &HashMap<String, usize>,
    param_indices: &HashMap<String, usize>,
) -> Stack {
    match term {
        Term::App(terms) => terms
            .iter()
            .map(|term| compile_term(term, def_indices, param_indices))
            .enumerate()
            .flat_map(|(i, mut stack)| {
                if i != 0 {
                    stack.push(Op::App);
                }
                stack
            })
            .collect::<Vec<_>>(),
        Term::Var(var) => {
            let def = def_indices.get(var).cloned().map(Op::Def);
            let arg = || param_indices.get(var).cloned().map(Op::Arg);
            let op = def
                .or_else(arg)
                .unwrap_or_else(|| panic!("undefined variable: {}", var.clone()));
            vec![op]
        }
    }
}

pub fn eval(mut state: State) -> State {
    state
}

pub fn show_state(state: &State) -> String {
    show_stack(&state.names, &mut state.stack.iter().copied().rev())
        .map(|term| show_term(&term))
        .unwrap_or("".to_string())
}

pub fn show_stack<I>(names: &[String], stack: &mut I) -> Option<Term>
where
    I: Iterator<Item = Op>,
{
    let l = show_op(names, stack.next()?)?;
    let mut terms = vec![l.clone()];
    while let Some(term) = show_stack(names, stack) {
        terms.push(term);
    }
    if terms.len() == 1 {
        Some(l)
    } else {
        Some(Term::App(terms))
    }
}

fn show_op(names: &[String], op: Op) -> Option<Term> {
    match op {
        Op::App => None,
        Op::Def(i) | Op::Arg(i) => Some(Term::Var(names[i].clone())),
    }
}
