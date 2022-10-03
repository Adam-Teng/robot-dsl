use std::fs;
use std::io::{self, BufRead, Write};
use std::process::exit;

use robot_dsl::{error::Error, interpreter::Interpreter, parser::Parser, scanner::Scanner};

struct Dsl {
    interpreter: Interpreter,
}

impl Dsl {
    fn new() -> Self {
        Dsl {
            interpreter: Interpreter::new(),
        }
    }

    fn run_file(&mut self, path: &str) -> Result<(), Error> {
        let source = fs::read_to_string(path)?;
        self.run(source)
    }

    fn run_prompt(&mut self) -> Result<(), Error> {
        let stdin = io::stdin();
        loop {
            print!("> ");
            io::stdout().flush()?;
            let mut line = String::new();
            stdin.lock().read_line(&mut line)?;
            self.run(line)?;
        }
    }

    fn run(&mut self, source: String) -> Result<(), Error> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        let mut parser = Parser::new(tokens);
        let statements = parser.parse()?;
        /*
        if let Some(expression) = parser.calculate() {
            println!("{}", self.interpreter.interpret_cal(&expression)?);
        }
        */
        self.interpreter.interpret(&statements)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args: Vec<String> = std::env::args().collect();
    let mut dsl = Dsl::new();
    match args.as_slice() {
        [_, file] => match dsl.run_file(file) {
            Ok(_) => (),
            Err(Error::Runtime { .. }) => exit(70),
            Err(Error::Parse) => exit(65),
            Err(Error::Io(_)) => unimplemented!(),
        },
        [_] => dsl.run_prompt()?,
        _ => {
            eprintln!("Usage: robot-dsl [script]");
            exit(64)
        }
    }
    Ok(())
}
