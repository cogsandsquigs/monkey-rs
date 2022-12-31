use crate::{lexer::Lexer, parser::Parser};
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
        let lexer = Lexer::new(&line);
        let mut parser = Parser::new(lexer);

        loop {
            let parsed = parser.parse_program();

            match parsed {
                Ok(program) => {
                    writeln!(out, "{}", program)?;
                    break;
                }
                Err(errors) => {
                    writeln!(out, "{}", MONKEY_FACE)?;
                    writeln!(out, "Woops! We ran into some monkey business here!")?;
                    writeln!(out, " parser errors:")?;
                    for error in errors {
                        writeln!(out, "\t{}", error)?;
                    }
                }
            }
        }
    }
}
