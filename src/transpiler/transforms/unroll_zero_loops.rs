use crate::transpiler::ir::IRNode;

pub fn unroll_zero_loops(branch: Vec<IRNode>) -> Vec<IRNode> {
    let mut transformed: Vec<IRNode> = vec![];
    for node in branch {
        match node {
            IRNode::Loop { children } => match children.as_slice() {
                [IRNode::Add {
                    offset: 0,
                    value: 1,
                }]
                | [IRNode::Add {
                    offset: 0,
                    value: -1,
                }] => transformed.push(IRNode::Zero { offset: 0 }),
                _ => transformed.push(IRNode::Loop { children }),
            },
            _ => transformed.push(node),
        }
    }

    transformed
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::transpiler::ir::IRNode;
    use pretty_assertions::assert_eq;

    use super::unroll_zero_loops;

    #[test]
    fn test_defer() {
        let ir = vec![IRNode::Loop {
            children: vec![IRNode::Add {
                offset: 0,
                value: -1,
            }],
        }];

        let expected = vec![IRNode::Zero { offset: 0 }];

        let result = unroll_zero_loops(ir);

        assert_eq!(result, expected);
    }
}
