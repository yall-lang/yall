use std::fmt::Debug;
use std::iter::Peekable;

pub struct PeekWhile<'a, I, P>
where
	I: Iterator,
	P: FnMut(&I::Item) -> bool,
{
	peekable: &'a mut Peekable<I>,
	pred: P,
}

impl<'a, I, P> Iterator for PeekWhile<'a, I, P>
where
	I: Iterator,
	P: FnMut(&I::Item) -> bool,
	I::Item: Debug,
{
	type Item = I::Item;

	fn next(&mut self) -> Option<Self::Item> {
		// let next = self.peekable.next();
		// if next.as_ref().filter(|i| (self.pred)(i)).is_some() {
		// 	next
		// } else {
		// 	None
		// }

		let peeked = self.peekable.peek()?;

		if (self.pred)(peeked) {
			// dbg!(peeked);
			// dbg!(self.peekable.next())
			self.peekable.next()
		} else {
			None
		}
	}
}

pub fn peek_while<I, P>(peekable: &mut Peekable<I>, pred: P) -> PeekWhile<I, P>
where
	I: Iterator,
	P: FnMut(&I::Item) -> bool,
{
	PeekWhile { peekable, pred }
}

// impl<I> PeekingTakeWhile for I
// where
// 	I: Iterator,
// {
// 	fn peek_while<P>(self, pred: P) -> PeekWhile<Self, P>
// 	where
// 		P: FnMut(&I::Item) -> bool,
// 	{
// 		PeekWhile {
// 			peekable: self.peekable(),
// 			pred,
// 		}
// 	}
// }
