extern crate clap;
extern crate pest_ascii_tree;
#[macro_use]
extern crate pest_derive;

use clap::{App, Arg, SubCommand};

mod commands;
mod interpreter;
mod parser;
mod utils;

fn main() {
    let mut app = App::new("Brainfuck util")
        .version("1.0")
        .author("Valtýr Örn Kjartansson <valtyr@gmail.com>")
        .about("Parse, run and transpile Brainfuck")
        .subcommand(
            SubCommand::with_name("debug")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .help("Sets the input file to use")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("parse-tree")
                        .short("p")
                        .help("Output a parse-tree for the file"),
                ),
        )
        .subcommand(
            SubCommand::with_name("run")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .help("Sets the input file to use")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("promiscuous")
                        .short("p")
                        .help("Allow another process to spy on execution"),
                )
                .about("Interpret a Brainfuck program"),
        )
        .subcommand(
            SubCommand::with_name("transpile")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .help("Sets the input file to use")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .help("Output path")
                        .required(true)
                        .takes_value(true),
                )
                .about("Generate C from Brainfuck"),
        )
        .subcommand(SubCommand::with_name("spy").about("Spy on another instance's execution"));

    let matches = app.clone().get_matches();

    match matches.subcommand() {
        ("debug", Some(cmd)) => commands::debug::run(cmd),
        ("run", Some(cmd)) => commands::interpret::run(cmd),
        ("transpile", Some(cmd)) => commands::transpile::run(cmd),
        ("spy", Some(cmd)) => commands::spy::run(cmd),
        _ => {
            app.print_long_help().expect("");
            ()
        }
    };
}
