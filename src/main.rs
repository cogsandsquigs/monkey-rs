pub mod ast;
pub mod evaluator;
pub mod lexer;
pub mod object;
pub mod parser;
pub mod repl;
pub mod token;

use std::io::Result;
use whoami::username;

fn main() -> Result<()> {
    println!(
        "Hello, {}! This is the Monkey programming language!",
        username()
    );
    println!("Feel free to type in commands");

    repl::start(std::io::stdin(), std::io::stdout())?;

    Ok(())
}
