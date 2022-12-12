use crate::token::TokenType;
use std::io::{BufRead, BufReader, Read, Result, Write};

const PROMPT: &str = ">> ";
const MONKEY_FACE: &str = r#"
           __,__
   .--.  .-"     "-.  .--.
  / .. \/  .-. .-.  \/ .. \
 | |  '|  /   Y   \  |'  | |
 | \   \  \ 0 | 0 /  /   / |
  \ '- ,\.-"""""""-./, -' /
   ''-' /_   ^ ^   _\ '-''
       |  \._   _./  |
       \   \ '~' /   /
        '._ '-=-' _.'
           '-----'
"#;

/// Runs the REPL. Reads input from `inp`, and writes output to `out`.
pub fn start<I: Read, O: Write>(inp: I, mut out: O) -> Result<()> {
    let mut reader = BufReader::new(inp);

    loop {
        // Print the prompt.
        out.write_all(PROMPT.as_bytes())?;
        out.flush()?;

        // Read a line of input.
        let mut line = String::new();
        reader.read_line(&mut line)?;

        // Lex the line
        let mut lexer = super::lexer::Lexer::new(&line);

        loop {
            let token = lexer.next_token();
            if token.r#type == TokenType::EOF {
                break;
            }

            // Print the token.
            out.write_all(format!("{:?}\n", token).as_bytes())?;
            out.flush()?;
        }
    }
}
