mod scanner;
use crate::scanner::*;

use std::env;
use std::fs;
use std::process::exit;
use std::io::{self, BufRead, Write};

fn run_file(path: &str) -> Result<(), String> {
	match fs::read_to_string(path) {
		Err(msg) => return Err(msg.to_string()),
		Ok(contents) => return run(&contents),
	}
}

fn run(contents: &str) -> Result<(), String> {
	let mut scanner = Scanner::new(contents);
	let tokens = scanner.scan_tokens()?;

	for token in tokens {
		println!("{:?}", token);
	}
	return Ok(());
}

fn run_prompt() -> Result<(), String> {
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
		match run(&buffer) {
			Ok(_) => (),
			Err(msg) => println!("{}", msg),
		}

	}
}

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() > 2 {
		println!("Using: Recolon [script]");
		exit(64);
	} else if args.len() == 2 {
		match run_file(&args[1]) {
			Ok(_) => exit(0),
			Err(msg) => {
				println!("ERROR:\n{}", msg);
				exit(1);
			}
		}
	} else {
		match run_prompt() {
			Ok(_) => exit(0),
			Err(msg) => {
				println!("ERROR:\n{}", msg);
				exit(1);
			}
		}
	}
}
