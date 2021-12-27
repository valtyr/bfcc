use crate::transpiler::ir::IRNode;

pub fn fuse_add(branch: Vec<IRNode>) -> Vec<IRNode> {
    let mut transformed: Vec<IRNode> = vec![];
    for node in branch {
        match node {
            IRNode::Add {
                value: by1,
                offset: offset1,
            } => match transformed.last().cloned() {
                Some(IRNode::Add {
                    value: by2,
                    offset: offset2,
                }) => {
                    if offset1 != offset2 {
                        transformed.push(IRNode::Add {
                            value: by1,
                            offset: offset1,
                        });
                    } else {
                        transformed.pop();
                        transformed.push(IRNode::Add {
                            value: by1 + by2,
                            offset: offset1,
                        });
                    }
                }
                _ => {
                    transformed.push(IRNode::Add {
                        value: by1,
                        offset: offset1,
                    });
                }
            },
            IRNode::Loop { children } => {
                let transformed_children = fuse_add(children);
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
