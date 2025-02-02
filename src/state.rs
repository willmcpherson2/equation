use std::collections::{HashMap, HashSet};

use crate::{Def, Program, Term};

#[derive(Debug, Clone)]
pub struct State {
    pub names: Vec<String>,
    pub procs: Vec<Procedure>,
    pub stack: Stack,
    pub registers: Vec<Stack>,
}

#[derive(Debug, Clone)]
pub struct Procedure {
    pub arity: usize,
    pub body: Stack,
}

pub type Stack = Vec<Op>;

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Def(usize),
    Arg(usize),
    App(usize),
}

pub fn compile(prog: &Program) -> Result<State, String> {
    let names = prog
        .iter()
        .map(|def| def.name.clone())
        .collect::<Vec<String>>();

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

    let main = def_indices
        .get("main")
        .cloned()
        .ok_or_else(|| "no main function defined".to_string())?;

    let mut stack = Vec::with_capacity(bytes_to_capacity::<Op>(1_000_000));
    stack.push(Op::Def(main));

    let registers = (0..8)
        .map(|_| Vec::with_capacity(bytes_to_capacity::<Op>(1_000)))
        .collect();

    Ok(State {
        names,
        procs,
        stack,
        registers,
    })
}

fn bytes_to_capacity<T>(bytes: usize) -> usize {
    bytes / std::mem::size_of::<T>()
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
                    stack.insert(0, Op::App(stack.len()));
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

pub fn eval(state: &mut State) {
    while let Some(()) = eval_step(state) {}
}

fn eval_step(state: &mut State) -> Option<()> {
    let State {
        procs,
        stack,
        registers,
        ..
    } = state;

    let Op::Def(def) = stack.pop()? else {
        return None;
    };

    let Procedure { arity, body } = &procs[def];

    pop_args(def, stack, registers, *arity)?;
    push_args(stack, registers, body)
}

fn pop_args(def: usize, stack: &mut Stack, registers: &mut [Stack], arity: usize) -> Option<()> {
    for register in 0..arity {
        let Some(Op::App(length)) = stack.pop() else {
            restore_stack(def, stack, registers, register);
            return None;
        };

        let end = stack.len();
        let start = end - length;

        registers[register].clear();
        registers[register].extend_from_slice(&stack[start..end]);
        stack.truncate(start);
    }

    Some(())
}

fn push_args(stack: &mut Stack, registers: &[Stack], body: &Stack) -> Option<()> {
    for (i, op) in body.iter().copied().enumerate() {
        match op {
            Op::Def(def) => stack.push(Op::Def(def)),
            Op::Arg(arg) => stack.extend_from_slice(&registers[arg]),
            Op::App(length) => {
                let start = i - length;
                let end = i;
                let length = get_length(&body[start..end], registers);
                stack.push(Op::App(length));
            }
        }
    }

    Some(())
}

fn get_length(body: &[Op], registers: &[Stack]) -> usize {
    body.iter()
        .copied()
        .map(|op| match op {
            Op::Arg(arg) => registers[arg].len(),
            _ => 1,
        })
        .sum()
}

fn restore_stack(def: usize, stack: &mut Stack, registers: &[Stack], args: usize) {
    for register in (0..args).rev() {
        let arg = &registers[register];
        stack.extend_from_slice(arg);
        stack.push(Op::App(arg.len()));
    }
    stack.push(Op::Def(def));
}

pub fn show_stack(state: &State) -> String {
    let mut string = String::new();
    let mut close_parens = HashSet::new();

    for (i, op) in state.stack.iter().cloned().rev().enumerate() {
        match op {
            Op::Def(def) => string.push_str(&state.names[def]),
            Op::Arg(arg) => {
                let state = State {
                    stack: state.registers[arg].clone(),
                    ..state.clone()
                };
                string.push_str(&show_stack(&state));
            }
            Op::App(length) => {
                string.push(' ');
                if length > 1 {
                    close_parens.insert(i + length);
                    string.push('(');
                }
            }
        }
        if close_parens.remove(&i) {
            string.push(')');
        }
    }

    string
}
