# rust-lox

## A Rust implementation of the Lox language from the book [Crafting Interpreters](https://craftinginterpreters.com/)


## Usage

- 'cargo run' is all you need to run the REPL.
- 'cargo run  -- --src-path "file_path" will run the interpreter on the file.

- for more optional flags run ./rlox --help

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

Generates the following png image:

- This a block statement which containes everything else in this example,
  for more complex use cases we could have multiple images generated.
![AST](/images/lox_variable.png)
