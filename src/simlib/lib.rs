#![feature(collections)]
#![feature(convert)]
pub mod event;
pub mod importer;

use std::iter::Iterator;

pub struct SimStream<'s> {
	source: &'s mut Iterator<Item=event::Event>
}

impl<'s> Iterator for SimStream<'s> {
	type Item = event::Event;

	fn next(&mut self) -> Option<event::Event> {
		self.source.next()
	}
}

pub fn sim<'s, I>(events: &'s mut I) -> SimStream<'s>
	where I: Iterator<Item=event::Event>
{
	SimStream{source: events}
}

