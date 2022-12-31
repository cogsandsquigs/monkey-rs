# monkey-rs

My implementation of the Monkey language in rust

This is designed to help me to learn how programming languages are implemented, following [Writing An Interpreter in Go](https://interpreterbook.com/). Note that for the most part, this follows the code in the book quite literally, unless there is something that I think could be replaced to be more idiomatic (For example, using `enum`s instead of string literals to differentiate token types).

Where applicable, and to the best of my ability, I will provide comments describing what the code is exactly doing, and any changes that I have made that differentiate it in some way from the base code (besides it obviously needing to be adapted to rust).

One main difference from the original implementation is that this rust implementation explicity supports Unicode and UTF-8, instead of ASCII. This lead to some annoyances, but overall, it wasn't too hard to implement (especially given that rust natively supports UTF-8 strings).

Note that this may update slowly - I do things on my own time, at my own pace :p.

## Notable changes

One of the most significant changes that I made was the error handling. Specifically, making it more idiomatic in the Rust language. See, in the original implementation, Thorsten Ball chooses to return `nil` if the parsing of an object/structure/thing does not work. However, this can lead to dangerously unsafe code. Initially, I changed the return types to `Result<T, ()>`, as that is at least safe. Unfortunately, that does not lend us to very idiomatic code, as we append errors manually to our parser, which is both tiring and unsafe (as we could miss an error, causing the parser to go haywire down the line).

The solution to this is twofold: Firstly, change the return type to `Result<T, E>`, where `E` is a custom error type that the language currently uses. This means we have to catch every error that does occur, guaranteeing that we have to handle them.

The next change is that in the main parsing loop (in `parse_program`), every time we encounter an error, we

1. Append it to a collection of errors
2. _Synchronize_ our input, so that we are at a point in the input that we should feel safe parsing without raising uncountable amounts of errors in the process.

Specifically, with the synchronization process, I consumed input until we reached `;`, as this at least guarantees us that we are at a new line, and that we can continue parsing regularily.
