# embeddir

![Crates.io](https://img.shields.io/crates/v/embeddir)

A macro that embeds all files in a specified directory into the executable at compile-time.

Works similarly to `include_bytes!` macro.

## Usage example

```rust
#![feature(proc_macro_hygiene)]

fn main() {
	let dir = embeddir::embed!("examples/static");

	for (filename, contents) in &dir {
		println!("Filename: {}, Contents: \"{}\"",
        filename,
        std::str::from_utf8(contents).unwrap());
	}
}
```
