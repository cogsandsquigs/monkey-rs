# monkey-rs

My implementation of the Monkey language in rust

This is designed to help me to learn how programming languages are implemented, following [Writing An Interpreter in Go](https://interpreterbook.com/). Note that for the most part, this follows the code in the book quite literally, unless there is something that I think could be replaced to be more idiomatic (For example, using `enum`s instead of string literals to differentiate token types).

Where applicable, and to the best of my ability, I will provide comments describing what the code is exactly doing, and any changes that I have made that differentiate it in some way from the base code (besides it obviously needing to be adapted to rust).

One main difference from the original implementation is that this rust implementation explicity supports Unicode and UTF-8, instead of ASCII. This lead to some annoyances, but overall, it wasn't too hard to implement (especially given that rust natively supports UTF-8 strings).

Note that this may update slowly - I do things on my own time, at my own pace :p.
