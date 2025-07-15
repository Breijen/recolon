mod scanner;
mod expr;
mod stmt;
mod parser;
mod interpreter;
mod environment;
mod types;
mod literal_value;
mod packages;
mod package_manager;
mod error;

use crate::scanner::*;
use crate::parser::*;
use crate::interpreter::*;
use crate::error::RecolonError;

use std::env;
use std::fs;
use std::process::exit;
use std::io::{self, BufRead, Write};

fn run_file(path: &str) -> Result<(), RecolonError> {
	let mut interpreter = Interpreter::new();
	match fs::read_to_string(path) {
		Err(msg) => Err(RecolonError::io(format!("Failed to read file '{}': {}", path, msg), 1)),
		Ok(contents) => run(&mut interpreter, &contents),
	}
}

fn run(interpreter: &mut Interpreter, contents: &str) -> Result<(), RecolonError> {
	let mut scanner = Scanner::new(contents);
	let tokens = scanner.scan_tokens()?;

	let mut parser = Parser::new(tokens);
	let stmts = parser.parse()?;
	let _ = interpreter.interpret(stmts)?;

	Ok(())
}

fn run_prompt() -> Result<(), RecolonError> {
	let mut interpreter = Interpreter::new();
	loop {
		print!("> ");
		match io::stdout().flush() {
			Ok(_) => (),
			Err(_) => return Err(RecolonError::io("Could not flush stdout".to_string(), 1)),
		}

		let mut buffer = String::new();
		let stdin = io::stdin();
		let mut handle = stdin.lock();
		match handle.read_line(&mut buffer) {
			Ok(n) => {
				dbg!(n);
				if n <= 2 {
					return Ok(());
				} 
			},
			Err(_) => return Err(RecolonError::io("Could not read line".to_string(), 1)),
		}

		println!("ECHO: {}", buffer);
		match run(&mut interpreter, &buffer) {
			Ok(_) => (),
			Err(error) => println!("{}", error.format_error(Some(&buffer))),
		}
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() > 2 {
		println!("Usage: Recolon [script]");
		exit(64);
	} else if args.len() == 2 {
		match run_file(&args[1]) {
			Ok(_) => (),
			Err(error) => {
				let source_code = std::fs::read_to_string(&args[1]).ok();
				println!("{}", error.format_error(source_code.as_deref()));
			},
		}
	} else {
		match run_prompt() {
			Ok(_) => (),
			Err(error) => println!("{}", error),
		}
	}

	// Wait for user input before closing
	println!("Press Enter to exit...");
	let _ = io::stdout().flush();
	io::stdin().read_line(&mut String::new()).unwrap();
}
