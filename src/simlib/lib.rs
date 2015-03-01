use std::iter::Iterator;

pub struct Event<'a> {
	pub name: &'a str
}

pub fn moo<'a, I>(events: I)
	where I: Iterator<Item=&'a Event<'a>>
{
	for x in events {
		println!("event name: {}", x.name);
	}
}

