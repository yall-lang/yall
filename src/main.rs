use std::convert::Infallible;
use std::env;
use std::fs;
use std::iter::Peekable;
use std::str::Chars;

mod options;
mod peek_while;

use options::Options;
// use peek_while::peek_while;

#[allow(dead_code)]
#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
enum ExpressionKind {
	Block,
	List,
	Item,
	Null,
}

#[derive(Clone, Debug)]
enum Phrase {
	Expression(Expression),
	Identifier(String),
	Text(String),
	Number(String),
	Comment(String),
}

#[derive(Clone, Debug)]
enum ParseError {
	OhShit,
}

impl From<Infallible> for ParseError {
	fn from(_: Infallible) -> Self {
		unreachable!()
	}
}

#[derive(Clone, Debug)]
struct Parser<'a> {
	s: Peekable<Chars<'a>>,
	_row: u32,
	_col: u32,
}

impl<'a> Parser<'a> {
	fn parse_whitespace(&mut self) -> Result<(), Infallible> {
		while let Some(c) = self.s.peek() {
			if !c.is_whitespace() {
				break;
			}
			self.s.next();
		}
		Ok(())
	}

	fn parse_string(&mut self) -> Result<Phrase, ParseError> {
		// Consume quote
		assert_eq!(self.s.next(), Some('"'));

		let mut is_next_escaped = false;

		let mut string = String::new();

		while self.s.peek() != Some(&'"') {
			string.push(self.s.next().expect("Should be a character"));

			if is_next_escaped {
				is_next_escaped = false;
				continue;
			}

			if self.s.peek() == Some(&'\\') {
				is_next_escaped = true;
				continue;
			}
		}

		Ok(Phrase::Text(string))
	}

	fn parse_comment(&mut self) -> Result<Phrase, ParseError> {
		assert_eq!(self.s.next(), Some(';'));

		let mut body = String::new();

		while self.s.peek() != Some(&'\n') {
			if self.s.peek().is_none() {
				break;
			}

			body.push(self.s.next().expect("Should be a comment"))
		}

		Ok(Phrase::Comment(body))
	}

	fn parse_number(&mut self) -> Result<Phrase, ParseError> {
		let mut contains_point = false;

		let mut number = String::new();

		while let Some(c) = self.s.peek() {
			if !contains_point && *c == '.' {
				contains_point = true;
				number.push(*c);
				continue;
			}

			if !c.is_ascii_digit() {
				break;
			}

			number.push(*c);
		}

		Ok(Phrase::Number(number))
	}

	fn parse_identifier(&mut self) -> Result<Phrase, ParseError> {
		let mut identifier = String::new();

		while let Some(c) = self.s.peek() {
			if c.is_ascii_alphanumeric() || *c == '_' {
				identifier.push(*c);
			} else {
				break;
			}
		}

		Ok(Phrase::Identifier(identifier))
	}

	fn parse_phrase(&mut self) -> Result<Phrase, ParseError> {
		self.parse_whitespace()?;
		self.s.next();

		match self.s.peek().ok_or(ParseError::OhShit)? {
			'(' | '[' | '{' => self.parse_expression().map(Phrase::Expression),
			'"' => self.parse_string(),
			';' => self.parse_comment(),
			x if x.is_ascii_digit() => self.parse_number(),
			x if x.is_ascii_alphabetic() => self.parse_identifier(),
			_ => Err(ParseError::OhShit),
		}
	}

	fn parse_expression(&mut self) -> Result<Expression, ParseError> {
		self.parse_whitespace()?;

		if self.s.peek() == Some(&';') {
			return Ok(Expression::null(self.parse_comment()?));
		}

		let kind = match self.s.next().ok_or(ParseError::OhShit)? {
			'[' => ExpressionKind::List,
			'{' => ExpressionKind::Block,
			'(' => ExpressionKind::Item,
			c => unreachable!("unexpected character {}", c),
		};

		let mut values = Vec::new();

		while let Ok(phrase) = self.parse_phrase() {
			values.push(phrase);
		}

		match kind {
			ExpressionKind::Block => assert_eq!(self.s.next(), Some('}')),
			ExpressionKind::List => assert_eq!(self.s.next(), Some(']')),
			ExpressionKind::Item => assert_eq!(self.s.next(), Some(')')),
			ExpressionKind::Null => unreachable!(),
		}

		Ok(Expression { kind, values })
	}

	pub fn parse_program(&mut self) -> Result<Vec<Expression>, ParseError> {
		let mut program = Vec::new();
		self.parse_whitespace()?;

		while self.s.peek().is_some() {
			program.push(self.parse_expression()?);
			self.parse_whitespace()?;
		}

		Ok(program)
	}

	pub fn new(stream: Peekable<Chars<'a>>) -> Self {
		Self {
			s: stream,
			_row: 0,
			_col: 0,
		}
	}
}

fn main() -> Result<(), ParseError> {
	let options = env::args().skip(1).collect::<Options>();

	let source = fs::read_to_string(options.input).unwrap();
	let stream = source.chars().peekable();

	let mut parser = Parser::new(stream);

	let program = parser.parse_program().unwrap();

	if options.debug_parser {
		println!("{:?}", program);
	}

	Ok(())
}
