mod scanner;
mod expr;
mod stmt;
mod parser;
mod interpreter;
mod environment;

mod modules;
mod types;
mod literal_value;

use crate::scanner::*;
use crate::parser::*;
use crate::interpreter::*;

use std::env;
use std::fs;
use std::process::exit;
use std::io::{self, BufRead, Write};

fn run_file(path: &str) -> Result<(), String> {
	let mut interpreter = Interpreter::new();
	match fs::read_to_string(path) {
		Err(msg) => Err(msg.to_string()),
		Ok(contents) => run(&mut interpreter, &contents),
	}
}

fn run(interpreter: &mut Interpreter, contents: &str) -> Result<(), String> {
	let mut scanner = Scanner::new(contents);
	let tokens = scanner.scan_tokens()?;

	let mut parser = Parser::new(tokens);
	let stmts = parser.parse()?;
	let _ = interpreter.interpret(stmts)?;

	return Ok(());
}

fn run_prompt() -> Result<(), String> {
	let mut interpreter = Interpreter::new();
	loop {
		print!("> ");
		match io::stdout().flush() {
			Ok(_) => (),
			Err(_) => return Err("Could not flush stdout".to_string()),
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
			Err(_) => return Err("Couldnt read line".to_string()),
		}

		println!("ECHO: {}", buffer);
		match run(&mut interpreter, &buffer) {
			Ok(_) => (),
			Err(msg) => println!("{}", msg),
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
			Err(msg) => println!("ERROR:\n{}", msg),
		}
	} else {
		match run_prompt() {
			Ok(_) => (),
			Err(msg) => println!("ERROR:\n{}", msg),
		}
	}

	// Wait for user input before closing
	println!("Press Enter to exit...");
	let _ = io::stdout().flush();
	io::stdin().read_line(&mut String::new()).unwrap();
}
