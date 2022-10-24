use crate::util::peek_while::peek_while;
use crate::util::peek_while::PeekWhile;
use std::iter::Peekable;
use std::str::Chars;

mod expression;
mod phrase;
pub use expression::parse_expression;
pub use expression::Expression;
pub use expression::ExpressionKind;
pub use phrase::Phrase;
use phrase::*;

pub struct Parser<I>
where
	I: Iterator<Item = char>,
{
	s: Peekable<I>,
	location: Location,
}

impl<I> Iterator for Parser<I>
where
	I: Iterator<Item = char>,
{
	type Item = char;

	fn next(&mut self) -> Option<Self::Item> {
		let next = self.s.next();

		if let Some('\n') = next {
			self.location.next_line();
		} else {
			self.location.next_column();
		}

		next
	}
}

// Ideally these would be a single `impl` with a  `AsRef<str>` constraint, but the
// compiler complains about `str` not being `Sized`.
impl<'a> From<&'a str> for Parser<Chars<'a>> {
	fn from(s: &'a str) -> Self {
		Self {
			s: s.chars().peekable(),
			location: Default::default(),
		}
	}
}
impl<'a> From<&'a String> for Parser<Chars<'a>> {
	fn from(s: &'a String) -> Self {
		Self {
			s: s.chars().peekable(),
			location: Default::default(),
		}
	}
}

impl<I: Iterator<Item = char>> Parser<I> {
	pub fn peek(&mut self) -> Option<&char> {
		self.s.peek()
	}

	pub fn peek_while<P>(&mut self, pred: P) -> PeekWhile<I, P>
	where
		P: FnMut(&char) -> bool,
	{
		peek_while(&mut self.s, pred)
	}
}

struct Location {
	line: isize,
	column: isize,
}

impl Location {
	pub fn next_column(&mut self) {
		self.column += 1;
	}

	pub fn next_line(&mut self) {
		self.line += 1;
		self.column = 1;
	}
}

impl Default for Location {
	fn default() -> Self {
		Self { line: 1, column: 1 }
	}
}

pub fn parse_program(
	s: &mut Parser<impl Iterator<Item = char>>,
) -> miette::Result<Vec<Expression>> {
	let mut program = Vec::new();
	parse_whitespace(s)?;

	while s.peek().is_some() {
		program.push(parse_expression(s)?);
		parse_whitespace(s)?;
	}

	Ok(program)
}
