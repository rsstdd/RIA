//! 2.10 - Reading from files

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
// Open a file obj
// Wrap file Obj in a BufReader
fn main() {
    let f = File::open("readme.md").unwrap();
    let mut reader = BufReader::new(f);

    let mut line = String::new();

    loop {
        let len = reader.read_line(&mut line).unwrap();

        if len == 0 {
            break
        }

        println!("{} ({} bytes line)", line, len);
        line.truncate(0);
    }
}
