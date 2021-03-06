use clap::ArgMatches;

use crate::{interpreter, parser, utils};

#[cfg(not(target_arch = "wasm32"))]
use crate::utils::doubleagent::DoubleAgent;

pub fn run(info: &ArgMatches) {
    let source = utils::read_source(info.value_of("input").unwrap().to_owned());

    let mut interpreter = interpreter::new();

    #[cfg(not(target_arch = "wasm32"))]
    if info.is_present("promiscuous") {
        let doubleagent = DoubleAgent::new();
        interpreter.add_events_hook(doubleagent);
        println!("⏸ Waiting for running instance of bf-spy");
    }

    match parser::parse_and_build_ast(&source) {
        Ok(ast) => interpreter.run(&ast),
        Err(e) => print!("{}", e),
    }
}
