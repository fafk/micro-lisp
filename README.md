# micro-lisp interpreter in Rust

Tiny lispesque language - everybody is writing their lisp iterpreters in Python, so I wrote mine in Rust. 

### Features:
✅ Branching (`(if (> 1 2) (do something) (do something_else))`)

✅ Iterations (`while (> i 10) (do stuff))`)

✅ Integers (i32)

✅ Arithmetic operations (`+` `-` `*`)

✅ Variables (`(set x 10)`)

✅ Printing to stdout (`(print hello)`)

### Interpretation steps:
* Chop up an input file with the lang source code into lexical units (tokens)
* Perform "parsing" phase by creating a "tree" by nesting `Vec`s according to the parentheses
* Evaluate the node of the tree

Every function (if, while, do, ...) returns a value.

Running the program:
`cargo run -- ./examples/loop.mlsp`
