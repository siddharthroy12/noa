# Grammar

## Statement grammar

```
program         -> declaration* EOF;
declaration     -> var_decl | statement;
var_decl        -> var IDENTIFIER ("=" statement)? ";";
statement       -> expr_statement | print_statement;
expr_statement  -> expression ";";
print_statement -> "print" expression ";";
```

## Expression grammar

```
expression     -> ternary ;
ternary        -> comparison ("?" comparison ":" comparison)?
equality       -> comparison ( ( "!=" | "==" ) comparison )* ;
comparison     -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           -> factor ( ( "-" | "+" ) factor )* ;
factor         -> unary ( ( "/" | "*" ) unary )* ;
unary          -> ( "!" | "-" ) unary
               | primary ;
primary        -> NUMBER | STRING | "true" | "false" | "nil"
               | "(" comma_operator ")" | IDENTIFIER;
comma_operator -> expression ("," expression)*;
```
