use clap::ArgMatches;

use crate::{
    parser::{self, ast::Node},
    utils,
};

fn c_for_node(line_vec: &mut Vec<String>, node: Node, indent: u16) {
    let spaces = std::iter::repeat(" ")
        .take((indent * 2) as usize)
        .collect::<String>();

    match node {
        Node::Forward => line_vec.push(spaces + "++ptr;"),
        Node::Backward => line_vec.push(spaces + "--ptr;"),
        Node::Increment => line_vec.push(spaces + "++*ptr;"),
        Node::Decrement => line_vec.push(spaces + "--*ptr;"),
        Node::Output => line_vec.push(spaces + "putchar(*ptr);"),
        Node::Input => line_vec.push(spaces + "*ptr = getchar();"),
        Node::Loop { children } => {
            line_vec.push(spaces.clone() + "while (*ptr) {");
            for child in children {
                c_for_node(line_vec, child, indent + 1);
            }
            line_vec.push(spaces + "}");
        }
    };
}

fn transpile(ast: Vec<Node>) -> String {
    let mut code: Vec<String> = vec![];

    for node in ast {
        c_for_node(&mut code, node, 1);
    }

    code.join("\n")
}

pub fn run(info: &ArgMatches) {
    let source = utils::read_source(info.value_of("input").unwrap().to_owned());

    let ast = match parser::parse_and_build_ast(&source) {
        Ok(ast) => ast,
        Err(e) => {
            print!("{}", e);
            return;
        }
    };

    let body = transpile(ast);

    let transpiled = format!(
        r#"#include <stdio.h>

char array[30000] = {{0}};
char *ptr = array;

int main(int argc, char* argv[]) {{
{}
}}
"#,
        body
    );

    utils::write_file(info.value_of("output").unwrap().to_owned(), transpiled);
}
