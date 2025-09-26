# Grammar

## Statement grammar

```
program         -> declaration* EOF;
declaration     -> var_decl | func_decl | statement;
var_decl        -> var IDENTIFIER ("=" statement)? ";";
func_decl       -> "fun" function;
function        -> IDENTIFIER "(" parameters? ")" block;
parameters      -> IDENTIFIER ( "," IDENTIFIER )* ;
statement       -> expr_statement | block | if | while | for | return;
return          _-> return expression? ";";
while           -> "while" "(" expression ")" statement;
for             -> "for" "(" (var_decl | expr_statement | ";") expression? ";" expression? ")" statement;
if              -> "if" "(" expression ")" statement ("else" statement)?;
block           -> "{" declaration* "}";
expr_statement  -> expression ";";
```

## Expression grammar

```
expression     -> assignment;
assignment     -> IDENTIFIER | key_access "=" (assignment | ternary);
or             -> and ("or" and)*;
and            -> ternary ("and" ternary)*;
ternary        -> equality ("?" equality ":" equality)?;
equality       -> comparison ( ( "!=" | "==" ) comparison )* ;
comparison     -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           -> factor ( ( "-" | "+" ) factor )* ;
factor         -> unary ( ( "/" | "*" ) unary )* ;
unary          -> ( "!" | "-" ) unary
               | call  | key_access;
call           -> key_access ("(" arguments? ")")*;
arguments      -> expression ("," expression)*;
key_access      -> primary "[" expression "]";
primary        -> NUMBER | STRING | "true" | "false" | "nil"| IDENTIFIER | table
               | "(" comma_operator ")";
comma_operator -> expression ("," expression)*;
table          -> "{" (key_value ",")* "}";
key_value      -> STRING ":" expression;
```
