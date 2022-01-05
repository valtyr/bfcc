use crate::{parser::ast::Node, transpiler::c::c_for_node};

mod c;
mod ir;
mod transforms;

pub fn transpile(ast: Vec<Node>) -> String {
    let mut transformed = transforms::from_ast_to_ir(&ast);
    transformed = transforms::fuse_add(transformed);
    // transformed = transforms::mul_loop_optimization(transformed);
    transformed = transforms::unroll_zero_loops(transformed);
    transformed = transforms::defer_movements(transformed);
    transformed = transforms::fuse_movements(transformed);

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
