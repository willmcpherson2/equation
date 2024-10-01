# The Equation Calculus

The equation calculus is a minimalist programming language. It consists only of top-level definitions, applications and variables:

```
true x y = x;
false x y = y;
not x = x false true;
main = not true;
```

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

```haskell
true x y = x;
false x y = y;
not x = x false true;
main = not true;
```

However, the equation calculus is untyped, so it's not an exact subset of Haskell.
