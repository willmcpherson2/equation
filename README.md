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

### Compilation

We apply the following transformations:

1. Intern variable names, replacing strings with indices: `Def(n)` for the nth definition and `Arg(n)` for the nth argument
2. Convert parameter lists to a simple `arity` count
3. Flatten nested applications into stacks consisting of `Def(n)`, `Arg(n)` and `App(n)` (application of the next `n` elements)

```rust
State {
    names: [
        "true",
        "false",
        "not",
        "main",
    ],
    procs: [
        // true x y = x;
        Procedure { arity: 2, body: [Arg(0)] },
        // false x y = y;
        Procedure { arity: 2, body: [Arg(1)] },
        // not x = x false true;
        Procedure { arity: 1, body: [Def(0), App(1), Def(1), App(1), Arg(0)] },
        // main = not true;
        Procedure { arity: 0, body: [Def(0), App(1), Def(2)] },
    ],
    stack: [
        // main
        Def(3)
    ],
    registers: [],
}
```

### Evaluation

To evaluate our program, we follow these steps:

1. Pop a `Def(n)` off the top of the `stack`
2. Index `procs[n]` to get a `Procedure`
3. Fill the registers based on the `arity` of the procedure
4. Copy the `body` of the procedure onto the top of the stack, mapping `Arg(n)` to the corresponding register and recalculating lengths of arguments for `App(n)`

### Memory allocation

The evaluation steps don't require any allocations unless space runs out in one of the arrays. With preallocation, some programs won't allocate any memory after startup.

### Data locality

We only push/pop the top of stack (end of the array), so this is very cache-friendly.

### Example

Let's evaluate this program:

```
true x y = x;
false x y = y;
not x = x false true;
main = not true;
```

```rust
State {
    // main
    stack: [Def(3)],
    registers: []
}

// pop `main` from the stack
// `main` has 0 arguments, so don't pop anything
// `main = not true`, so push that to the stack

State {
    // not true
    stack: [Def(0), App(1), Def(2)],
    registers: []
}

// pop `not` from the stack
// `not` has 1 argument, pop it from the stack to a register
// `not x = x false true`, so push that to the stack
// `x` is the first register which is `true`

State {
    // true false true
    stack: [Def(0), App(1), Def(1), App(1), Def(0)],
    registers: [
        // true
        [Def(0)]
    ]
}

// pop `true` from the stack
// `true` has 2 arguments, pop them from the stack to registers
// `true x y = x`, so push that to the stack
// `x` is the first register which is `false`
// `y` is the first register which is `true`

State {
    // false
    stack: [Def(1)],
    registers: [
        // false
        [Def(1)],
        // true
        [Def(0)]
    ]
}
```
