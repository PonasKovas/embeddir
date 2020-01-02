//! A simple crate for embedding all files in a directory into the executable.
//!
//! Works similarly to `std::include_bytes!` macro.
//!
//! For this macro to work, you must have `proc_macro_hygiene` feature enabled.
//!
//!```#![feature(proc_macro_hygiene)]```
//!
//! The files in the directory are not searched recursively, so if you have a directory tree like this:
//!
//! ```text
//! Dir1/
//! ├──File1
//! ├──File2
//! └──Dir2/
//!    └──File3
//! ```
//!
//! And specify the `Dir1` directory to look for files in, `File1` and `File2` will be embedded into your executable
//! but `File3` will not.

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{Expr, Lit};
use quote::quote;
use std::path::{Path, PathBuf};
use std::env::var;
use std::fs::read_dir;


/// Takes a literal path and gives back a `HashMap` mapping filenames to their contents
///
/// The path can be either absolute or relative to the location of your crate's `Cargo.toml`
///
/// # Usage example:
///
/// ```
/// use embeddir::embed;
/// use std::collections::HashMap;
///
/// let dir: HashMap<&str, &[u8]> = embed!("some/path");
///
/// let file_contents = dir["file.txt"];
/// ```

#[proc_macro]
pub fn embed(input: TokenStream) -> TokenStream {
    let directory = match syn::parse_macro_input!(input as Expr) {
        Expr::Lit(lit) => match lit.lit {
            Lit::Str(lit) => lit.value(),
            _ => panic!("expected a string literal"),
        },
        _ => panic!("expected a string literal"),
    };

    // Make a Path
    let mut directory = PathBuf::from(directory);
    if directory.is_relative() {
        directory = Path::new(&var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set!")).join(directory);
    }

    // Check if the path is valid
    if !directory.is_dir() {
        panic!("{:?} does not exist or is not a directory!", directory);
    }

    let mut filenames = Vec::new();
    let mut paths = Vec::new();

    // Iterate over the files in this directory
    for file in read_dir(&directory).expect("Couldn't read the contents of the specified directory") {
        let file = file.unwrap();
        if !file.file_type().unwrap().is_file() {
            continue;
        }
        let filename = file.file_name();
        filenames.push(filename.clone().into_string().unwrap());
        paths.push(directory.join(filename).to_str().unwrap().to_owned());
    }

    TokenStream::from(quote!{{
        let mut map = std::collections::HashMap::new();
        #( map.insert(#filenames, &include_bytes!(#paths)[..]); )*
        map
    }})
}
