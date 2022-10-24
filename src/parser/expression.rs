use miette::miette;

use super::phrase::*;
use super::Parser;

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Expression {
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
pub enum ExpressionKind {
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

pub fn parse_expression(s: &mut Parser<impl Iterator<Item = char>>) -> miette::Result<Expression> {
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
