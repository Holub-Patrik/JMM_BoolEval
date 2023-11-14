#![allow(unused_imports, unused_variables)]

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use rust_ast::Compile;

use rust_ast::interpret::Interpreter as Engine;

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    println!("Calculator prompt. Expressions are line evaluated.");
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                match Engine::from_source(&line) {
                    Ok(result) => println!("{}", result),
                    Err(e) => eprintln!("{}", e),
                };
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
