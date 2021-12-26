use std::time::Instant;

use clap::ArgMatches;

use crate::{parser, utils};

pub fn run(info: &ArgMatches) {
    let now = Instant::now();

    let source = utils::read_source(info.value_of("input").unwrap().to_owned());

    if info.is_present("parse-tree") {
        match parser::parse(&source) {
            Ok(result) => pest_ascii_tree::print_ascii_tree(Ok(result)),
            Err(e) => print!("{}", e),
        }
        return;
    }

    match parser::parse_and_build_ast(&source) {
        Ok(_result) => {
            let elapsed = now.elapsed().as_micros();
            if elapsed >= 1000 {
                println!("Parsed in {} ms", elapsed as f32 / 1000.0)
            } else {
                println!("Parsed in {} µs", now.elapsed().as_micros())
            }
        }
        Err(e) => print!("{}", e),
    }
}
