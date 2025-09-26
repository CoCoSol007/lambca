# Lambda (λ)

Lambda is a very tiny [DSL](https://en.wikipedia.org/wiki/Domain-specific_language) to manipulate [lambda calculus terms](https://en.wikipedia.org/wiki/Lambda_calculus). The syntax is very simple. And the project itself is very minimalist.

## Syntax

- Variable: x, y, foo …
- Abstraction: \x. M (function taking x and returning M)
- Application: (M N) (apply function M to argument N)

### Available commands:

- Define a term: let <name> = <lambda_term>
- Evaluate a term: eval <lambda_term> (beta-reduce until no more steps & print the result)

## Examples

```plaintext
let id = \x. x
eval (id z)                     // => z

let self = \s. (s s)
eval (self self)                // => (\s.(s s) \s.(s s))

let k = \x.\y. x
eval ((k a) b)                  // => a

let flip = \f.\x.\y. ((f y) x)
eval (((flip id) p) q)          // => (q p)

let chain = \f.\g.\x. (f (g x))
eval (((chain id) id) r)        // => r
```

you can find more "concrete" examples in the [`examples` folder](examples).

## Running

todo

## Installation

todo