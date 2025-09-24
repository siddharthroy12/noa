# Rlox

Rlox is a toy programming language written in Rust.

Rlox is made to learn how to write interpreters. I have been reading [Crafting Interpreters](https://craftinginterpreters.com/) to learn how to build interpreters.

To learn how to use this langauge, refer to the [sample.rlox](./sample.rlox) file, which contains all the featues this langauge has.

## Building

1. Make sure that you are Rust complier installed and all of it's dependencies
2. Run `cargo build` or `cargo run` to build or run the interpreter

## Usage

`rlox [script]` will run the script.

`rlox` will run the interpreter in REPL mode.

## Grammar

If you are interested in the grammar definitions of this language you can read it [here](./Grammar.md).

## TODO

- Add break expression;
- Add ++ -- expression;
- Add continue expression;
