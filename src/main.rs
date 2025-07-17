use std::{env, path::Path};

use crate::{scanner::Scanner, token::Token};
mod scanner;
mod token;

fn main() -> std::io::Result<()> {
    match env::args().nth(1) {
        Some(file) => run_file(&file),
        None => run_prompt(),
    }
}

fn run_file(path: impl AsRef<Path>) -> std::io::Result<()> {
    let s = std::fs::read_to_string(path)?;
    run(&s);

    Ok(())
}

fn run_prompt() -> std::io::Result<()> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    loop {
        print!("> ");
        stdin.read_line(&mut buffer)?;

        if buffer.is_empty() {
            break;
        }

        run(&buffer);
    }

    Ok(())
}

fn run(source: &str) {
    let mut scanner = Scanner::new(source.to_owned());
    let tokens: Vec<Token> = scanner.scan_tokens();

    for token in tokens {
        println!("{token:?}");
    }
}
