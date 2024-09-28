use std::collections::HashMap;

use crate::{show_term, Def, Program, Term};

#[derive(Debug, Clone)]
pub struct State {
    def_names: Vec<String>,
    def_arities: Vec<usize>,
    def_stacks: Vec<Stack>,
    main_stack: Stack,
    arg_stack: Stack,
}

pub type Stack = Vec<Op>;

#[derive(Debug, Clone, Copy)]
pub enum Op {
    App,
    Def(usize),
    Arg(usize),
}

pub fn compile(program: &Program) -> State {
    let def_names = program.iter().map(|def| def.name.clone()).collect();

    let def_arities = program.iter().map(|def| def.params.len()).collect();

    let def_indices: HashMap<String, usize> = program
        .iter()
        .enumerate()
        .map(|(i, def)| (def.name.clone(), i))
        .collect();
    let def_stacks: Vec<Stack> = program
        .iter()
        .map(|def| compile_def(def, &def_indices))
        .collect();

    let main_stack = program
        .iter()
        .position(|def| def.name == "Main")
        .and_then(|index| def_stacks.get(index).cloned())
        .unwrap_or(vec![]);

    let arg_stack = vec![];

    State {
        def_names,
        def_arities,
        def_stacks,
        main_stack,
        arg_stack,
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

pub fn eval(state: &mut State) {}

pub fn show_state(state: &State) -> String {
    show_stack(
        &state.def_names,
        &mut state.main_stack.iter().copied().rev(),
    )
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
