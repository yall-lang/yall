use miette::miette;

use super::parse_expression;
use super::Expression;
use super::Parser;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Phrase {
	Expression(Expression),
	Identifier(String),
	Text(String),
	Number(String),
	Comment(String),
}

pub fn parse_whitespace(s: &mut Parser<impl Iterator<Item = char>>) -> miette::Result<()> {
	while let Some(c) = s.peek() {
		if !c.is_whitespace() {
			break;
		}
		s.next();
	}

	Ok(())
}

pub fn parse_string(s: &mut Parser<impl Iterator<Item = char>>) -> miette::Result<Phrase> {
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
		assert!(parse_string(&mut invalid.into()).is_err());
	}

	#[test]
	fn valid() {
		let hello = "\"hello!\"";
		assert_eq!(
			parse_string(&mut hello.into()).unwrap(),
			Phrase::Text("hello!".to_string())
		);
	}

	#[test]
	fn escape_quote() {
		let hello = r#""hello \"buddy\"!""#;
		assert_eq!(
			parse_string(&mut hello.into()).unwrap(),
			Phrase::Text("hello \\\"buddy\\\"!".to_string())
		);
	}

	#[test]
	fn stops() {
		let hello = "\"hello!\"🏁";
		let mut parser = hello.into();
		assert_eq!(
			parse_string(&mut parser).unwrap(),
			Phrase::Text("hello!".to_string())
		);
		assert_eq!(parser.next(), Some('🏁'));
	}
}

pub fn parse_comment(s: &mut Parser<impl Iterator<Item = char>>) -> miette::Result<Phrase> {
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
			parse_comment(&mut hello.into()).unwrap(),
			Phrase::Comment(" hello!".to_string())
		);
	}

	#[test]
	fn stops() {
		let hello = "; hello!\n🏁";
		let mut parser = hello.into();
		assert_eq!(
			parse_comment(&mut parser).unwrap(),
			Phrase::Comment(" hello!".to_string())
		);
		assert_eq!(parser.next(), Some('🏁'));
	}
}

pub fn parse_number(s: &mut Parser<impl Iterator<Item = char>>) -> miette::Result<Phrase> {
	let mut contains_point = false;

	let number = s
		.peek_while(|&c: &char| {
			if !contains_point && c == '.' {
				contains_point = true;
				return true;
			}

			c.is_ascii_digit()
		})
		.collect();

	Ok(Phrase::Number(number))
}

pub fn parse_text_identifier(s: &mut Parser<impl Iterator<Item = char>>) -> miette::Result<Phrase> {
	let identifier = s
		.peek_while(|&c| c.is_ascii_alphanumeric() || c == '_')
		.collect();

	Ok(Phrase::Identifier(identifier))
}

static OPERATOR_CHARACTERS: [char; 13] = [
	'*', '+', '-', '/', '<', '>', '=', '!', '$', '|', '?', '^', '~',
];
pub fn parse_operator_identifier(
	s: &mut Parser<impl Iterator<Item = char>>,
) -> miette::Result<Phrase> {
	let identifier = s.peek_while(|c| OPERATOR_CHARACTERS.contains(c)).collect();

	Ok(Phrase::Identifier(identifier))
}

pub fn parse_type(s: &mut Parser<impl Iterator<Item = char>>) -> miette::Result<()> {
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

pub fn parse_phrase(s: &mut Parser<impl Iterator<Item = char>>) -> miette::Result<Phrase> {
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
