#![feature(proc_macro_hygiene)]

extern crate embeddir;

fn main() {
	let dir = embeddir::embed!("examples/static");

	for (filename, contents) in &dir {
		println!("Filename: {}, Contents: \"{}\"", filename, std::str::from_utf8(contents).unwrap());
	}
}
