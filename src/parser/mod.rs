pub extern crate pest;
pub use pest::{iterators::Pairs, Parser};

use self::ast::Node;

pub mod ast;

#[derive(Parser)]
#[grammar = "grammar/brainfuck.pest"]
pub struct BFParser;

pub fn parse(source: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
    let result = BFParser::parse(Rule::file, source)?;

    Ok(result)
}

pub fn build_ast(pairs: Pairs<Rule>) -> Vec<Node> {
    let mut ast = vec![];

    for pair in pairs {
        match pair.as_rule() {
            // TODO:
            // The first two rules here result in an unnecessary heap allocation for
            // the ast variable. Should be extracted to a separate wrapper function,
            // as they only occur at the top level
            Rule::file => {
                return build_ast(pair.into_inner());
            }
            Rule::program => {
                return build_ast(pair.into_inner());
            }
            Rule::forward => ast.push(Node::Forward),
            Rule::backward => ast.push(Node::Backward),
            Rule::increment => ast.push(Node::Increment),
            Rule::decrement => ast.push(Node::Decrement),
            Rule::input => ast.push(Node::Input),
            Rule::output => ast.push(Node::Output),
            Rule::while_loop => ast.push(Node::Loop {
                children: build_ast(pair.into_inner().clone()),
            }),
            _ => {
                debug_assert!(false, "Unmatched {:?}\n", pair.as_rule());
            }
        }
    }

    ast
}

pub fn parse_and_build_ast(source: &str) -> Result<Vec<Node>, pest::error::Error<Rule>> {
    let result = parse(source)?;
    let ast = build_ast(result);

    Ok(ast)
}
