use std::{collections::HashMap, ops::Range};

use crate::{show_term, Def, Program, Term};

#[derive(Debug, Clone)]
pub struct State {
    pub names: Vec<String>,
    pub procs: Vec<Procedure>,
    pub stack: Stack,
    pub args: Stack,
    pub arg_ranges: Vec<Range<usize>>,
}

#[derive(Debug, Clone)]
pub struct Procedure {
    pub arity: usize,
    pub body: Stack,
}

pub type Stack = Vec<Op>;

#[derive(Debug, Clone, Copy)]
pub enum Op {
    App,
    Def(usize),
    Arg(usize),
}

pub fn compile(prog: &Program) -> Result<State, String> {
    let names = prog.iter().map(|def| def.name.clone()).collect();

    let def_indices = prog
        .iter()
        .enumerate()
        .map(|(i, def)| (def.name.clone(), i))
        .collect::<HashMap<String, usize>>();
    let procs = prog
        .iter()
        .map(|def| {
            Ok(Procedure {
                arity: def.params.len(),
                body: compile_def(def, &def_indices)?,
            })
        })
        .collect::<Result<Vec<Procedure>, String>>()?;

    let main = prog
        .iter()
        .position(|def| def.name == "main")
        .ok_or_else(|| "no main function defined".to_string())?;

    Ok(State {
        names,
        procs,
        stack: vec![Op::Def(main)],
        args: vec![],
        arg_ranges: vec![],
    })
}

fn compile_def(def: &Def, def_indices: &HashMap<String, usize>) -> Result<Stack, String> {
    let param_indices: HashMap<String, usize> = def
        .params
        .iter()
        .enumerate()
        .map(|(i, param)| (param.clone(), i))
        .collect();
    let mut stack = compile_term(&def.term, def_indices, &param_indices)?;
    stack.reverse();
    Ok(stack)
}

fn compile_term(
    term: &Term,
    def_indices: &HashMap<String, usize>,
    param_indices: &HashMap<String, usize>,
) -> Result<Stack, String> {
    match term {
        Term::App(terms) => Ok(terms
            .iter()
            .map(|term| compile_term(term, def_indices, param_indices))
            .collect::<Result<Vec<Stack>, String>>()?
            .into_iter()
            .enumerate()
            .flat_map(|(i, mut stack)| {
                if i != 0 {
                    stack.push(Op::App);
                }
                stack
            })
            .collect::<Vec<_>>()),
        Term::Var(var) => {
            let def = def_indices.get(var).cloned().map(Op::Def);
            let arg = || param_indices.get(var).cloned().map(Op::Arg);
            let op = def
                .or_else(arg)
                .ok_or_else(|| format!("undefined variable: {}", var.clone()))?;
            Ok(vec![op])
        }
    }
}

pub fn eval(mut state: State) -> State {
    while let Some(()) = eval_step(&mut state) {}
    state
}

fn eval_step(state: &mut State) -> Option<()> {
    println!("{}", show_stack(&state.names, &state.stack));

    let Some(Op::Def(i)) = state.stack.pop() else {
        return None;
    };

    let Procedure { arity, body } = &state.procs[i];

    state.args.clear();
    state.arg_ranges.clear();
    for _ in 0..*arity {
        get_arg(&mut state.stack, &mut state.args, &mut state.arg_ranges)?;
    }

    for op in body {
        match *op {
            Op::Arg(i) => {
                let arg_range = state.arg_ranges[i].clone();
                let arg = &state.args[arg_range];
                state.stack.extend_from_slice(arg);
            }
            op => state.stack.push(op),
        }
    }

    Some(())
}

fn get_arg(stack: &mut Stack, args: &mut Stack, arg_ranges: &mut Vec<Range<usize>>) -> Option<()> {
    let mut apps_needed = 0;
    let arg_length = stack.iter().rev().position(|op| {
        match op {
            Op::App => apps_needed -= 1,
            _ => apps_needed += 1,
        }
        apps_needed <= 0
    })?;

    let index = stack.len() - arg_length;
    let arg_start = args.len();
    let arg_end = arg_start + arg_length;
    args.extend(stack.drain(index..));
    arg_ranges.push(arg_start..arg_end);
    let _app = stack.pop();
    Some(())
}

pub fn show_stack(names: &[String], stack: &Stack) -> String {
    show_stack_impl(names, &mut stack.iter().copied().rev())
        .map(|term| show_term(&term))
        .unwrap_or("".to_string())
}

fn show_stack_impl<I>(names: &[String], stack: &mut I) -> Option<Term>
where
    I: Iterator<Item = Op>,
{
    let l = show_op(names, stack.next()?)?;
    let mut terms = vec![l.clone()];
    while let Some(term) = show_stack_impl(names, stack) {
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
