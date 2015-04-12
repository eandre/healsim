extern crate simlib;

fn main() {
	let events = vec!(simlib::Event{name: "hi"}, simlib::Event{name: "there"});
	{
		let iter = &mut events.iter();
		let sim_stream = simlib::sim(iter);
		for event in sim_stream {
			println!("Got event {}", event.name)
		}
	}
}