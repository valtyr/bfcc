use crate::{parser::ast::Node, transpiler::ir::IRNode};

pub fn from_ast_to_ir(branch: &Vec<Node>) -> Vec<IRNode> {
    let mut transformed: Vec<IRNode> = vec![];

    for node in branch {
        match node {
            Node::Loop { children } => {
                let transformed_children = from_ast_to_ir(children);
                transformed.push(IRNode::Loop {
                    children: transformed_children,
                })
            }
            Node::Forward => transformed.push(IRNode::Move(1)),
            Node::Backward => transformed.push(IRNode::Move(-1)),
            Node::Increment => transformed.push(IRNode::Add {
                value: 1,
                offset: 0,
            }),
            Node::Decrement => transformed.push(IRNode::Add {
                value: -1,
                offset: 0,
            }),
            Node::Output => transformed.push(IRNode::Output { offset: 0 }),
            Node::Input => transformed.push(IRNode::Input { offset: 0 }),
        }
    }

    return transformed;
}
