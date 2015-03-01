extern crate simlib;

fn main() {
	let events : Vec<simlib::Event> = vec!(simlib::Event{name: "hi"}, simlib::Event{name: "there"});
	simlib::moo(events.iter());
}