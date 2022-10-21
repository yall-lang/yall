use std::iter::Peekable;

#[derive(Clone, Debug)]
struct Lexer<'a, I>
where
	I: Iterator<Item = char>,
{
	s: &'a mut Peekable<I>,
	location: Location,
}

impl<'a, I> Lexer<'a, I>
where
	I: Iterator<Item = char>,
{
	fn new(s: &mut Peekable<I>) -> Self {
		Self {
			s,
			location: Default::default(),
		}
	}

	fn peek(&mut self) -> Option<<Self as Iterator>::Item> {
		self.s.peek()
	}

	fn peek_while<P>(&mut self, pred: P)
	where
		P: FnMut(&char) -> bool,
	{
		self.s.peek_while(pred)
	}
}

impl<'a, I> Iterator for Lexer<'a, I>
where
	I: Iterator<Item = char>,
{
	type Item = LexicalChar;

	fn next(&mut self) -> Option<Self::Item> {
		let c = self.s.next()?;
		let location = self.location.advance_for(c);

		LexicalChar(self.s.next(), location)
	}
}

#[derive(Copy, Clone, Debug)]
struct LexicalChar(char, Location);

#[derive(Copy, Clone, Debug)]
struct Location {
	line: isize,
	column: isize,
}

impl Location {
	fn advance_for(&mut self, c: char) {
		if (c == '\n') {
			self.location.line += 1;
			self.location.column = 0;
		} else {
			self.location.column += 1;
		}
	}
}

impl Default for Location {
	fn default() -> Self {
		Self { line: 1, column: 0 }
	}
}
