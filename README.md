# λambda

Lambda is a very tiny DSL to manipulate lambda terms. The syntax is very simple. And the project itself is very minimalist.

## Syntax

The syntax of λambda is based on the untyped lambda calculus. Here are some basic constructs:

- **Variables**: `x`, `y`, `foo`, `bar`, `val2` ...
- **Abstraction**: `λx. M` (function that takes `x` and returns `M`)
- **Application**: `(M N)` (apply function `M` to argument `N`)

The construction of a lambda term is by combining these constructs.

In order to interact with the DSL, you can use the following instructions:
- **Define a term**: `let <name> = <lambda_term>`
- **Evaluate a term**: `eval <lambda_term>`
When I say "evaluate", I mean performing beta-reduction until no more reductions can be made (and print the result).

## Examples

```plaintext
let id = \x. x
let const = (id id)
eval (\y. (const y))
```

This will output `\y. y`.

you can find more examples in the [`examples` folder](examples).

## Running

todo

## Installation

todo