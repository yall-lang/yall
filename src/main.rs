use miette::miette;
use std::env;
use std::fs;

mod options;
mod parser;
mod util {
	pub mod peek_while;
}

use options::Options;

fn main() -> miette::Result<()> {
	let options = env::args().skip(1).collect::<Options>();

	let source = fs::read_to_string(options.input).or(Err(miette!("failed to read input file")))?;
	let program = parser::parse_program(&mut (&source).into())?;

	if options.debug_parser {
		println!("{:#?}", program);
	}

	Ok(())
}
