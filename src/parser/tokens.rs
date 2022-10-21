fn parse_whitespace(s: &mut Peekable<impl Iterator<Item = char>>) -> Result<(), Infallible> {
	while let Some(c) = s.peek() {
		if !c.is_whitespace() {
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

fn parse_comment(s: &mut Peekable<impl Iterator<Item = char>>) -> Result<Phrase, ParseError> {
	assert_eq!(s.next(), Some(';'));
	let body = s.take_while(|&c| c != '\n').collect();
	Ok(Phrase::Comment(body))
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
