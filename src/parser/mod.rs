mod lexer;
mod tokens;

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
	pub fn from_originator(c: char) -> Self {
		match c {
			'{' => Self::Block,
			'[' => Self::List,
			'(' => Self::Item,
			c => unreachable!("unexpected character {}", c),
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

#[derive(Clone, Debug)]
enum ParseError {
	OhShit,
}

impl From<Infallible> for ParseError {
	fn from(_: Infallible) -> Self {
		unreachable!()
	}
}

fn parse_phrase(s: &mut Peekable<impl Iterator<Item = char>>) -> Result<Phrase, ParseError> {
	parse_whitespace(s)?;

	match s.peek().ok_or(ParseError::OhShit)? {
		'(' | '[' | '{' => parse_expression(s).map(Phrase::Expression),
		'"' => parse_string(s),
		';' => parse_comment(s),
		x if x.is_ascii_digit() => parse_number(s),
		x if x.is_ascii_alphabetic() => parse_identifier(s),
		_ => Err(ParseError::OhShit),
	}
}

fn parse_expression(
	s: &mut Peekable<impl Iterator<Item = char>>,
) -> Result<Expression, ParseError> {
	parse_whitespace(s)?;

	if s.peek() == Some(&';') {
		return Ok(Expression::null(parse_comment(s)?));
	}

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

	assert_eq!(s.next(), Some(kind.terminator()));

	Ok(Expression { kind, values })
}

fn parse_program(
	s: &mut Peekable<impl Iterator<Item = char>>,
) -> Result<Vec<Expression>, ParseError> {
	let mut program = Vec::new();
	parse_whitespace(s)?;

	while s.peek().is_some() {
		program.push(parse_expression(s)?);
		parse_whitespace(s)?;
	}

	Ok(program)
}
