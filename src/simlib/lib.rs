use std::iter::Iterator;

pub struct Event<'a> {
	pub name: &'a str
}

pub struct SimStream<'a> {
	source: &'a mut Iterator<Item=&'a Event<'a>>
}

impl<'a> Iterator for SimStream<'a> {
	type Item = &'a Event<'a>;

	fn next(&mut self) -> Option<&'a Event<'a>> {
		self.source.next()
	}
}

pub fn sim<'a, I>(events: &'a mut I) -> SimStream<'a>
	where I: Iterator<Item=&'a Event<'a>>
{
	SimStream{source: events}
}

