use std::convert::Infallible;
use std::env;
use std::fs;
use std::iter::Peekable;

mod options;
mod parser;
mod peek_while;

use options::Options;
use peek_while::peek_while;

fn main() -> Result<(), ParseError> {
	let options = env::args().skip(1).collect::<Options>();

	let source = fs::read_to_string(options.input).unwrap();
	let mut stream = source.chars().peekable();

	let program = parse_program(&mut stream).unwrap();
	if options.debug_parser {
		println!("{:?}", program);
	}

	Ok(())
}
