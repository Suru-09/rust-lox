# Lox grammar

## Bugs

- [.] Return values are further stringified, should be fixed.
  - in order to fix the return value issue, I have to make a workaround in the interpreter at execute_block because on error(a.k.a on return statement) we can only return a string at the moment, so I would have to parse the string to a value, which is not ideal.
- [ ] clock() function is not scoped properly, when trying to call it outside a function it will fail.
- [ ] Calling fibonacci recursively does not return correct values, I have negative values in the output, to be investigated.


## Initial grammar(to be updated)

-----------------

```lox
expression     → literal
               | unary
               | binary
               | grouping ;

literal        → NUMBER | STRING | "true" | "false" | "nil" ;
grouping       → "(" expression ")" ;
unary          → ( "-" | "!" ) expression ;
binary         → expression operator expression ;
operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
               | "+"  | "-"  | "*" | "/" ;
```

## Updated grammar

-----------------

```lox
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;
```
