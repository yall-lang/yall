use miette::miette;
use std::env;
use std::fs;
use std::iter::Peekable;

mod options;
mod peek_while;

use options::Options;
use peek_while::peek_while;

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

#[derive(Clone, Debug)]
enum Phrase {
	Expression(Expression),
	Identifier(String),
	Text(String),
	Number(String),
	Comment(String),
}

fn parse_whitespace(s: &mut Peekable<impl Iterator<Item = char>>) -> miette::Result<()> {
	while let Some(c) = s.peek() {
		if !c.is_whitespace() {
			break;
		}
		s.next();
	}

	Ok(())
}

fn parse_string(s: &mut Peekable<impl Iterator<Item = char>>) -> miette::Result<Phrase> {
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
	fn fail() {
		let invalid = "twat";
		assert!(dbg!(parse_string(&mut invalid.chars().peekable())).is_err());
	}
}

fn parse_comment(s: &mut Peekable<impl Iterator<Item = char>>) -> miette::Result<Phrase> {
	// Consume ;
	let start = s.next();
	if start != Some(';') {
		return Err(miette!("entered parse_comment with no comment to parse"));
	}

	let body = s.take_while(|&c| c != '\n').collect();
	Ok(Phrase::Comment(body))
}

fn parse_number(s: &mut Peekable<impl Iterator<Item = char>>) -> miette::Result<Phrase> {
	let mut contains_point = false;

	let number = peek_while(s, |&c| {
		if !contains_point && c == '.' {
			contains_point = true;
			return true;
		}

		c.is_ascii_digit()
	})
	.collect();

	Ok(Phrase::Number(number))
}

fn parse_identifier(s: &mut Peekable<impl Iterator<Item = char>>) -> miette::Result<Phrase> {
	let identifier = peek_while(s, |&c| c.is_ascii_alphanumeric() || c == '_').collect();

	Ok(Phrase::Identifier(identifier))
}

fn parse_phrase(s: &mut Peekable<impl Iterator<Item = char>>) -> miette::Result<Phrase> {
	parse_whitespace(s)?;

	match s.peek().ok_or(miette!("unexpected end of file"))? {
		'(' | '[' | '{' => parse_expression(s).map(Phrase::Expression),
		'"' => parse_string(s),
		';' => parse_comment(s),
		x if x.is_ascii_digit() => parse_number(s),
		x if x.is_ascii_alphabetic() => parse_identifier(s),
		_ => Err(miette!("unexpected character")),
	}
}

fn parse_expression(s: &mut Peekable<impl Iterator<Item = char>>) -> miette::Result<Expression> {
	parse_whitespace(s)?;

	if s.peek() == Some(&';') {
		return Ok(Expression::null(parse_comment(s)?));
	}

	let kind =
		ExpressionKind::from_initiator(s.next().ok_or(miette!("expected an expression here"))?)?;

	let mut values = Vec::new();
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

fn parse_program(s: &mut Peekable<impl Iterator<Item = char>>) -> miette::Result<Vec<Expression>> {
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

	let program = parse_program(&mut stream)?;
	if options.debug_parser {
		println!("{:?}", program);
	}

	Ok(())
}
