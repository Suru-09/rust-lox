# rust-lox

## A Rust implementation of the Lox language from the book [Crafting Interpreters](https://craftinginterpreters.com/).

This is a work in progress. I'm currently working on the interpreter, and I'll be adding the compiler later.

## Usage

- 'cargo run' is all you need to run the REPL.
- 'cargo run  \<filename\>' will run the interpreter on the file.

## TODO

- [ ] add support for lines & columns where errors occur.
- [ ] add support for block comments.
- [ ] add support for error recovery & synchronization.
- [ ] add tests for the interpreter.
- [ ] add suppport for break and continue in loops.
- [ ] add support for newlines and tabs in print statements(\n, \t).
- [x] add tests for the scanner.


## AST visualizer

The AST visualizer in my case is a visitor that constructs a dot file, from which we generate a png/ multiple pngs representing the AST.

For example, the following code:

```java
{
    var lox = 1 + 2 - 3 + 4 - 5 / 7 * 9 - 12.3 * (2.34 + 11);
    print lox;

    lox = "string";

    {
        var lox = " Hehehe another string in this block";
        print lox;
    }

    print lox;
}
```

Generates the following two pngs:

- This a block statement which containes everything else in this example,
  for more complex use cases we could have multiple images generated.
![AST](/images/lox_variable.png)
