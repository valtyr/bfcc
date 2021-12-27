use crate::transpiler::ir::IRNode;

pub fn fuse_movements(branch: Vec<IRNode>) -> Vec<IRNode> {
    let mut transformed: Vec<IRNode> = vec![];
    for node in branch {
        match node {
            IRNode::Move(n1) => match transformed.last().cloned() {
                Some(IRNode::Move(n2)) => {
                    transformed.pop();
                    transformed.push(IRNode::Move(n1 + n2));
                }
                _ => {
                    transformed.push(IRNode::Move(n1));
                }
            },
            IRNode::Loop { children } => {
                let transformed_children = fuse_movements(children);
                transformed.push(IRNode::Loop {
                    children: transformed_children,
                })
            }
            _ => {
                transformed.push(node);
            }
        }
    }
    transformed
}
