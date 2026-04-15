# calc

A toy programming language with a simple grammar, built in Rust.

> **Status:** Draft — this project is in early development. Currently it contains the grammar documentation and sample parsers.

## Language

calc supports variable assignment, basic arithmetic expressions (`+`, `-`, `*`, `/`), reading input (`@`), and printing output (`<`). Variables are declared implicitly via assignment (`:=`).

### Grammar

See [`docs/grammar.bnf`](docs/grammar.bnf) for the full BNF grammar.

| Syntax | Meaning |
|---|---|
| `x := <expr>` | Assign expression to variable |
| `@ x` | Read input into variable |
| `> x` | (reserved) |
| `< <expr>` | Print expression |

### Example Program

```
@ x
@ y
sum := x + y
diff := x - y
product := x * y
< sum
< diff
< product * (sum + 2)
```

## Project Structure

- **`docs/`** — Grammar specification (BNF)
- **`calc_parser/`** — Parser for the calc language (WIP)
- **`sample_nom_parsers/`** — Sample parsers using [nom](https://crates.io/crates/nom)
