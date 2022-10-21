use std::convert::Infallible;
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

#[derive(Clone, Debug)]
enum ExpressionKind {
	Block,
	List,
	Item,
}

#[derive(Clone, Debug)]
enum Phrase {
	Expression(Expression),
	Identifier(String),
	Text(String),
	Number(String),
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

fn parse_whitespace_and_comment(s: &mut Peekable<impl Iterator<Item = char>>) -> Result<(), Infallible> {
	let mut is_in_comment = false;

	while let Some(c) = s.peek() {
		if c == &';' {
			is_in_comment = true;
		} else if c == &'\n' && is_in_comment {
			is_in_comment = false;
		}

		if !is_in_comment && !c.is_whitespace() {
			break;
		}

		s.next();
	}

	Ok(())
}

fn parse_string(s: &mut Peekable<impl Iterator<Item = char>>) -> Result<Phrase, ParseError> {
	// Consume quote
	assert_eq!(s.next(), Some('"'));

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

fn parse_number(s: &mut Peekable<impl Iterator<Item = char>>) -> Result<Phrase, ParseError> {
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

fn parse_identifier(s: &mut Peekable<impl Iterator<Item = char>>) -> Result<Phrase, ParseError> {
	let identifier = peek_while(s, |&c| c.is_ascii_alphanumeric() || c == '_').collect();

	Ok(Phrase::Identifier(identifier))
}

fn parse_phrase(s: &mut Peekable<impl Iterator<Item = char>>) -> Result<Phrase, ParseError> {
	parse_whitespace_and_comment(s)?;

	match s.peek().ok_or(ParseError::OhShit)? {
		'(' | '[' | '{' => parse_expression(s).map(Phrase::Expression),
		'"' => parse_string(s),
		x if x.is_ascii_digit() => parse_number(s),
		x if x.is_ascii_alphabetic() => parse_identifier(s),
		_ => Err(ParseError::OhShit),
	}
}

fn parse_expression(
	s: &mut Peekable<impl Iterator<Item = char>>,
) -> Result<Expression, ParseError> {
	parse_whitespace_and_comment(s)?;

	let kind = match s.next().ok_or(ParseError::OhShit)? {
		'[' => ExpressionKind::List,
		'{' => ExpressionKind::Block,
		'(' => ExpressionKind::Item,
		c => unreachable!("unexpected character {}", c),
	};

	let mut values = Vec::new();

	while let Ok(phrase) = parse_phrase(s) {
		values.push(phrase);
	}

	match kind {
		ExpressionKind::Block => assert_eq!(s.next(), Some('}')),
		ExpressionKind::List => assert_eq!(s.next(), Some(']')),
		ExpressionKind::Item => assert_eq!(s.next(), Some(')')),
	}

	Ok(Expression { kind, values })
}

fn parse_program(
	s: &mut Peekable<impl Iterator<Item = char>>,
) -> Result<Vec<Expression>, ParseError> {
	let mut program = Vec::new();
	parse_whitespace_and_comment(s)?;

	while s.peek().is_some() {
		program.push(parse_expression(s)?);
		parse_whitespace_and_comment(s)?;
	}

	Ok(program)
}

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
