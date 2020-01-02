#![feature(proc_macro_hygiene)]

extern crate embed_dir;

fn main() {
	let dir = embed_dir::embed_dir!("examples/static");
	
	for (filename, contents) in &dir {
		println!("Filename: {}, Contents: \"{}\"", filename, std::str::from_utf8(contents).unwrap());
	}
}
