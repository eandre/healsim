#![feature(collections)]
extern crate simlib;

fn main() {
    let mut it = simlib::importer::wcl::import(String::from_str("abc"), 1);
	{
		let sim_stream = simlib::sim(&mut it);
		for event in sim_stream {
			println!("Got event {}", event.name)
		}
	}
}
