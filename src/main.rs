use miette::miette;
use std::env;
use std::fs;
use std::iter::Peekable;

mod options;
mod peek_while;

use options::Options;
use peek_while::peek_while;

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
struct Expression {
	kind: ExpressionKind,
	values: Vec<Phrase>,
}

impl Expression {
	pub fn null(value: Phrase) -> Self {
		Self {
			kind: ExpressionKind::Null,
			values: vec![value],
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ExpressionKind {
	Block,
	List,
	Item,
	Null,
}

impl ExpressionKind {
	pub fn from_initiator(c: char) -> miette::Result<Self> {
		match c {
			'{' => Ok(Self::Block),
			'[' => Ok(Self::List),
			'(' => Ok(Self::Item),
			c => Err(miette!(
				"unexpected character {}, expected an expression",
				c
			)),
		}
	}

	pub fn terminator(&self) -> char {
		match self {
			Self::Block => '}',
			Self::List => ']',
			Self::Item => ')',
			Self::Null => unreachable!(),
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Phrase {
	Expression(Expression),
	Identifier(String),
	Text(String),
	Number(String),
	Comment(String),
}

struct Location {
	row: isize,
	col: isize,
}

impl Location {
	pub fn next_col(&mut self) {
		self.col += 1;
	}

	pub fn next_row(&mut self) {
		self.row += 1;
		self.col = 1;
	}

}

impl Default for Location {
	fn default() -> Self {
		Self { row: 1, col: 1 }
	}
}

struct Parser<'a, I>
where
	I: Iterator<Item = char>,
{
	s: &'a mut Peekable<I>,
	loc: Location,
}

impl<'a, I: Iterator<Item = char>> Iterator for Parser<'a, I> {
	type Item = char;

	fn next(&mut self) -> Option<Self::Item> {
		let next = self.s.next();

		if let Some('\n') = next {
			self.loc.next_row();
		} else {
			self.loc.next_col();
		}

		next
	}
}

impl<'a, I: Iterator<Item = char>> Parser<'a, I> {
	pub fn new(stream: &'a mut Peekable<I>) -> Self {
		Self {
			s: stream,
			loc: Location::default(),
		}
	}

	pub fn peek(&mut self) -> Option<&char> {
		self.s.peek()
	}
}

fn parse_whitespace(s: &mut Parser<impl Iterator<Item = char>>) -> miette::Result<()> {
	while let Some(c) = s.peek() {
		if !c.is_whitespace() {
			break;
		}
		s.next();
	}

	Ok(())
}

fn parse_string(s: &mut Parser<impl Iterator<Item = char>>) -> miette::Result<Phrase> {
	// Consume "
	let quote = s.next();
	if quote != Some('"') {
		return Err(miette!("entered parse_string with no string to parse"));
	}

	let mut is_next_escaped = false;

	let text = s
		.take_while(|&c| {
			if is_next_escaped {
				is_next_escaped = false;
				return true;
			}

			if c == '\\' {
				is_next_escaped = true;
				return true;
			}

			c != '"'
		})
		.collect();

	Ok(Phrase::Text(text))
}

#[cfg(test)]
mod parse_string_tests {
	use super::*;

	#[test]
	fn invalid() {
		let invalid = "invalid";
		assert!(parse_string(&mut Parser::new(&mut invalid.chars().peekable())).is_err());
	}

	#[test]
	fn valid() {
		let hello = "\"hello!\"";
		assert_eq!(
			parse_string(&mut Parser::new(&mut hello.chars().peekable())).unwrap(),
			Phrase::Text("hello!".to_string())
		);
	}

	#[test]
	fn escape_quote() {
		let hello = r#""hello \"buddy\"!""#;
		assert_eq!(
			parse_string(&mut Parser::new(&mut hello.chars().peekable())).unwrap(),
			Phrase::Text("hello \\\"buddy\\\"!".to_string())
		);
	}

	#[test]
	fn stops() {
		let hello = "\"hello!\"🏁";
		let mut s = hello.chars().peekable();
		assert_eq!(
			parse_string(&mut Parser::new(&mut s)).unwrap(),
			Phrase::Text("hello!".to_string())
		);
		assert_eq!(s.next(), Some('🏁'));
	}
}

fn parse_comment(s: &mut Parser<impl Iterator<Item = char>>) -> miette::Result<Phrase> {
	// Consume ;
	let start = s.next();
	if start != Some(';') {
		return Err(miette!("entered parse_comment with no comment to parse"));
	}

	let body = s.take_while(|&c| c != '\n').collect();
	Ok(Phrase::Comment(body))
}

#[cfg(test)]
mod parse_comment_tests {
	use super::*;

	#[test]
	fn no_new_line() {
		let hello = "; hello!";
		assert_eq!(
			parse_comment(&mut Parser::new(&mut hello.chars().peekable())).unwrap(),
			Phrase::Comment(" hello!".to_string())
		);
	}

	#[test]
	fn stops() {
		let hello = "; hello!\n🏁";
		let mut peekable = hello.chars().peekable();
		let mut s = Parser::new(&mut peekable);
		assert_eq!(
			parse_comment(&mut s).unwrap(),
			Phrase::Comment(" hello!".to_string())
		);
		assert_eq!(s.next(), Some('🏁'));
	}
}

fn parse_number(s: &mut Parser<impl Iterator<Item = char>>) -> miette::Result<Phrase> {
	let mut contains_point = false;

	let number = peek_while(s.s, |&c: &char| {
		if !contains_point && c == '.' {
			contains_point = true;
			return true;
		}

		c.is_ascii_digit()
	})
	.collect();

	Ok(Phrase::Number(number))
}

fn parse_text_identifier(s: &mut Parser<impl Iterator<Item = char>>) -> miette::Result<Phrase> {
	let identifier = peek_while(s.s, |&c| c.is_ascii_alphanumeric() || c == '_').collect();

	Ok(Phrase::Identifier(identifier))
}

static OPERATOR_CHARACTERS: [char; 13] = [
	'*', '+', '-', '/', '<', '>', '=', '!', '$', '|', '?', '^', '~',
];
fn parse_operator_identifier(s: &mut Parser<impl Iterator<Item = char>>) -> miette::Result<Phrase> {
	let identifier = peek_while(s.s, |c| OPERATOR_CHARACTERS.contains(c)).collect();

	Ok(Phrase::Identifier(identifier))
}

fn parse_type(s: &mut Parser<impl Iterator<Item = char>>) -> miette::Result<()> {
	let prefix = s.take(2).collect::<String>();
	if prefix != "::" {
		return Err(miette!("entered parse_type with no type to parse"));
	}

	// This function unfortunately can't quite just use `parse_phrase`, because that
	// would allow weird things like `a::b::c` since `parse_phrase` calls us. We need some
	// mechanism to prevent annotating types with types. I guess that could be done as a
	// separate validation step, but I feel like it shouldn't make it past the parser.
	parse_text_identifier(s)?;

	Ok(())
}

fn parse_phrase(s: &mut Parser<impl Iterator<Item = char>>) -> miette::Result<Phrase> {
	parse_whitespace(s)?;

	let phrase = match s.peek().ok_or(miette!("unexpected end of file"))? {
		'(' | '[' | '{' => parse_expression(s).map(Phrase::Expression),
		'"' => parse_string(s),
		';' => parse_comment(s),
		x if x.is_ascii_digit() => parse_number(s),
		x if x.is_ascii_alphabetic() => parse_text_identifier(s),
		x if OPERATOR_CHARACTERS.contains(x) => parse_operator_identifier(s),
		_ => Err(miette!("unexpected character")),
	};

	if s.peek() == Some(&':') {
		parse_type(s)?;
	};

	phrase
}

fn parse_expression(s: &mut Parser<impl Iterator<Item = char>>) -> miette::Result<Expression> {
	parse_whitespace(s)?;

	if s.peek() == Some(&';') {
		return Ok(Expression::null(parse_comment(s)?));
	}

	let kind =
		ExpressionKind::from_initiator(s.next().ok_or(miette!("expected an expression here"))?)?;

	let mut values = vec![];
	while let Ok(phrase) = parse_phrase(s) {
		values.push(phrase);
	}

	let terminator = s.next();
	if terminator != Some(kind.terminator()) {
		return Err(miette!(
			"expected {} to terminate expression, got {:?}",
			kind.terminator(),
			terminator
		));
	}

	Ok(Expression { kind, values })
}

fn parse_program(s: &mut Parser<impl Iterator<Item = char>>) -> miette::Result<Vec<Expression>> {
	let mut program = Vec::new();
	parse_whitespace(s)?;

	while s.peek().is_some() {
		program.push(parse_expression(s)?);
		parse_whitespace(s)?;
	}

	Ok(program)
}

fn main() -> miette::Result<()> {
	let options = env::args().skip(1).collect::<Options>();

	let source = fs::read_to_string(options.input).or(Err(miette!("failed to read input file")))?;
	let mut stream = source.chars().peekable();

	let mut parser = Parser::new(&mut stream);
	let program = parse_program(&mut parser).unwrap();

	if options.debug_parser {
		println!("{:#?}", program);
	}

	Ok(())
}
