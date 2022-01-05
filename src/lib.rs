extern crate clap;
extern crate pest_ascii_tree;
#[macro_use]
extern crate pest_derive;

use wasm_bindgen::prelude::*;

mod interpreter;
mod parser;
mod transpiler;
mod utils;

use crate::parser::{parse, build_ast, ast::Node};
use crate::transpiler::{IRNode, transforms, generate_c};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct ParsingResult {
    ast: Vec<Node>,
    ir: Vec<IRNode>,
    c: String,
}


#[wasm_bindgen]
pub fn process(source: &JsValue) -> Result<JsValue, JsValue> {
    let source_string = source.as_string().unwrap();
    let parsed = match parse(&source_string) {
        Ok(result) => result,
        Err(error) => {
            return Err(JsValue::from_str(&format!("{}", error)))
        }
    };

    let ast = build_ast(parsed);
    let ir = transforms::from_ast_to_ir(&ast);
    let c = generate_c(&ir);

    Ok(JsValue::from_serde(&ParsingResult {
        ast,
        ir,
        c
    }).unwrap())
}