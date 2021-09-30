use std::{env, error::Error, fs::read_to_string};

use lexer::Lexer;

use crate::lexer::TokenKind;

mod lexer;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: mimir [path]");
        Ok(())
    } else {
        let path = &args[1];
        let source = read_to_string(path)?;
        let mut lexer = Lexer::new(&source);

        loop {
            let token = lexer.scan_token();
            println!("{:?}", token);
            if token.kind == TokenKind::EndOfFile {
                break;
            }
        }
        Ok(())
    }
}
