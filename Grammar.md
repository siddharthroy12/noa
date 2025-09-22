# Grammar

## Statement grammar

```
program         -> declaration* EOF;
declaration     -> var_decl | statement;
var_decl        -> var IDENTIFIER ("=" statement)? ";";
statement       -> expr_statement | print_statement | block;
block           -> "{" declaration* "}"
expr_statement  -> expression ";";
print_statement -> "print" expression ";";
```

## Expression grammar

```
expression     -> assignment;
assignment     -> IDENTIFIER "=" (assignment | ternary);
ternary        -> equality ("?" equality ":" equality)?;
equality       -> comparison ( ( "!=" | "==" ) comparison )* ;
comparison     -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           -> factor ( ( "-" | "+" ) factor )* ;
factor         -> unary ( ( "/" | "*" ) unary )* ;
unary          -> ( "!" | "-" ) unary
               | primary ;
primary        -> NUMBER | STRING | "true" | "false" | "nil"| IDENTIFIER
               | "(" comma_operator ")";
comma_operator -> expression ("," expression)*;
```
