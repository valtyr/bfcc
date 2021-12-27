use crate::parser::ast::Node;

use super::ir::IRNode;

pub fn transform(branch: &Vec<Node>) -> Vec<IRNode> {
    let mut transformed: Vec<IRNode> = vec![];

    for node in branch {
        match node {
            Node::Loop { children } => {
                let transformed_children = transform(children);
                transformed.push(IRNode::Loop {
                    children: transformed_children,
                })
            }
            Node::Forward => match transformed.last().cloned() {
                Some(IRNode::ForwardBackward(count)) => {
                    transformed.pop();
                    transformed.push(IRNode::ForwardBackward(count + 1));
                }
                _ => {
                    transformed.push(IRNode::ForwardBackward(1));
                }
            },
            Node::Backward => match transformed.last().cloned() {
                Some(IRNode::ForwardBackward(count)) => {
                    transformed.pop();
                    transformed.push(IRNode::ForwardBackward(count - 1));
                }
                _ => {
                    transformed.push(IRNode::ForwardBackward(-1));
                }
            },
            Node::Increment => match transformed.last().cloned() {
                Some(IRNode::IncrementDecrement(count)) => {
                    transformed.pop();
                    transformed.push(IRNode::IncrementDecrement(count + 1));
                }
                _ => {
                    transformed.push(IRNode::IncrementDecrement(1));
                }
            },
            Node::Decrement => match transformed.last().cloned() {
                Some(IRNode::IncrementDecrement(count)) => {
                    transformed.pop();
                    transformed.push(IRNode::IncrementDecrement(count - 1));
                }
                _ => {
                    transformed.push(IRNode::IncrementDecrement(-1));
                }
            },
            Node::Output => transformed.push(IRNode::Output),
            Node::Input => transformed.push(IRNode::Input),
        }
    }

    return transformed;
}
