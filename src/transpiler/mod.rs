use crate::{
    parser::ast::Node,
    transpiler::{c::c_for_node, transform::transform},
};

mod c;
mod ir;
mod transform;

pub fn transpile(ast: Vec<Node>) -> String {
    let transformed = transform(&ast);

    let mut code: Vec<String> = vec![];

    for node in transformed {
        c_for_node(&mut code, node, 1);
    }

    let body = code.join("\n");

    format!(
        r#"#include <stdio.h>

char array[30000] = {{0}};
char *ptr = array;

int main(int argc, char* argv[]) {{
{}
}}
"#,
        body
    )
}
