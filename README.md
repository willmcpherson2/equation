# The Equation Calculus

The equation calculus is a minimalist programming language. It consists only of top-level definitions, applications and variables:

```
true x y = x;
false x y = y;
not x = x false true;
main = not true;
```

The `main` definition will be evaluated to `false`.

## Lambda calculus

The equation calculus is the lambda calculus but with named functions instead of anonymous functions.

The downside of this is that you must define a function to pass it to another function. Also there's no environment capture.

The upside is that definitions make code much easier to read and write. Definitions are also recursive.

## Combinator calculus

The equation calculus is the combinator calculus but with no predefined combinators.

```
I x = x;
K x y = x;
S f g x = f x (g x);
main = S K I I;
```

## Haskell

It's called the equation calculus because it uses Haskell's equation syntax. In fact, our first example was valid Haskell code:

```
true x y = x;
false x y = y;
not x = x false true;
main = not true;
```

However, the equation calculus is untyped, so it's not an exact subset of Haskell.

## Implementation

The equation calculus is very easy to implement efficiently. For example, take this program:

```
true x y = x;
false x y = y;
not x = x false true;
main = not true;
```

We apply the following transformations:

1. Intern variable names, replacing strings with indices: `Def(n)` for the nth definition and `Arg(n)` for the nth argument
2. Convert parameter lists to a simple `arity` count
3. Flatten nested applications into stacks consisting of `Arg(n)`, `Def(n)` and `App` to apply a function.

```rust
State {
    procs: [
        // true x y = x;
        Procedure { arity: 2, body: [Arg(0)] },
        // false x y = y;
        Procedure { arity: 2, body: [Arg(1)] },
        // not x = x false true;
        Procedure { arity: 1, body: [App, Def(0), App, Def(1), Arg(0)] },
        // main = not true;
        Procedure { arity: 0, body: [App, Def(0), Def(2)] },
    ],
    // main
    stack: [Def(3)],
    args: [],
    arg_ranges: [],
}
```

Because an argument can consist of multiple instructions, we will use `Arg(n)` to index `arg_ranges` to get a `start..end` and then use that to index `args`.

To evaluate our program, we follow these steps:

1. Pop a `Def(n)` off the top of the `stack`
2. Index `procs[n]` to get a `Procedure`
3. Fill the argument stacks based on the `arity` of the procedure
4. Copy the `body` of the procedure onto the top of the stack, mapping `Arg(n)` to the corresponding argument

```rust
// stack: main
stack: [Def(3)]
args: []
arg_ranges: []

// popped: main
// definition: main = not true;
stack: []
args: []
arg_ranges: []

// arguments: []
stack: []
args: []
arg_ranges: []

// stack: not true
stack: [App, Def(0), Def(2)]
args: []
arg_ranges: []

// popped: not
// definition: not x = x false true;
stack: [App, Def(0)]
args: []
arg_ranges: []

// arguments: [true]
stack: []
args: [Def(0)]
arg_ranges: [0..1]

// stack: true false true
stack: [App, Def(0), App, Def(1), Def(0)]
args: [Def(0)]
arg_ranges: [0..1]

// popped: true
// definition: true x y = x;
stack: [App, Def(0), App, Def(1)]
args: []
arg_ranges: []

// arguments: [false, true]
stack: []
args: [Def(1), Def(0)]
arg_ranges: [0..1, 1..2]

// stack: false
stack: [Def(1)]
args: [Def(1), Def(0)]
arg_ranges: [0..1, 1..2]

// popped: false
// definition: false x y = y;
stack: []
args: []
arg_ranges: []

// could not get arguments. terminating.
stack: []
args: []
arg_ranges: []
```
