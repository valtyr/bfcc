use std::fs;
use std::io::Read;
use std::num::Wrapping;

pub mod doubleagent;

pub fn read_source(path: String) -> String {
    return fs::read_to_string(path).expect("Something went wrong reading the file");
}

pub fn write_file(path: String, contents: String) {
    fs::write(path, contents).expect("Something went wrong writing to the file");
}

pub fn read_byte() -> Option<u8> {
    std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
}

pub fn overflow_add(lhs: u8, rhs: u8) -> u8 {
    let x = Wrapping(lhs);
    let y = Wrapping(rhs);

    // This is technically undefined behavior in rust
    // but it's low overhead, and it's easy. Might
    // implement proper emulation logic later.
    (x + y).0
}
