use clap::ArgMatches;

use crate::{parser, transpiler, utils};

pub fn run(info: &ArgMatches) {
    let source = utils::read_source(info.value_of("input").unwrap().to_owned());

    let ast = match parser::parse_and_build_ast(&source) {
        Ok(ast) => ast,
        Err(e) => {
            print!("{}", e);
            return;
        }
    };

    let transpiled = transpiler::transpile(ast);

    if info.is_present("output") {
        utils::write_file(info.value_of("output").unwrap().to_owned(), transpiled);
    } else {
        println!("{}", transpiled);
    }
}
