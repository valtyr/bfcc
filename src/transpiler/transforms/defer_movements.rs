use crate::transpiler::ir::IRNode;

pub fn defer_movements(branch: Vec<IRNode>) -> Vec<IRNode> {
    let mut transformed: Vec<IRNode> = vec![];
    let mut accumulated_offset = 0;
    for node in branch {
        match node {
            IRNode::Move(delta) => accumulated_offset += delta,
            IRNode::Add { value, offset } => transformed.push(IRNode::Add {
                value,
                offset: offset + accumulated_offset,
            }),
            IRNode::Mul { value, offset } => transformed.push(IRNode::Mul {
                value,
                offset: offset + accumulated_offset,
            }),
            IRNode::Output { offset } => transformed.push(IRNode::Output {
                offset: offset + accumulated_offset,
            }),
            IRNode::Input { offset } => transformed.push(IRNode::Output {
                offset: offset + accumulated_offset,
            }),
            IRNode::Loop { children } => {
                if accumulated_offset != 0 {
                    transformed.push(IRNode::Move(accumulated_offset));
                    accumulated_offset = 0;
                }

                let transformed_children = defer_movements(children);
                transformed.push(IRNode::Loop {
                    children: transformed_children,
                })
            }
            IRNode::Zero { offset } => transformed.push(IRNode::Zero {
                offset: offset + accumulated_offset,
            }),
        }
    }

    if accumulated_offset != 0 {
        transformed.push(IRNode::Move(accumulated_offset));
    }

    transformed
}

#[cfg(test)]
mod tests {
    use crate::transpiler::ir::IRNode;
    use pretty_assertions::assert_eq;

    use super::defer_movements;

    #[test]
    fn test_defer() {
        let ir = vec![
            IRNode::Move(3),
            IRNode::Add {
                offset: 0,
                value: -1,
            },
            IRNode::Move(1),
            IRNode::Add {
                offset: 0,
                value: 1,
            },
        ];

        let expected = vec![
            IRNode::Add {
                offset: 3,
                value: -1,
            },
            IRNode::Add {
                offset: 4,
                value: 1,
            },
            IRNode::Move(4),
        ];

        let result = defer_movements(ir);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_defer_with_loop() {
        let ir = vec![
            IRNode::Move(3),
            IRNode::Add {
                offset: 0,
                value: -1,
            },
            IRNode::Move(1),
            IRNode::Output { offset: 0 },
            IRNode::Loop { children: vec![] },
            IRNode::Move(-2),
            IRNode::Add {
                offset: 0,
                value: 10,
            },
            IRNode::Move(2),
        ];

        let expected = vec![
            IRNode::Add {
                offset: 3,
                value: -1,
            },
            IRNode::Output { offset: 4 },
            IRNode::Move(4),
            IRNode::Loop { children: vec![] },
            IRNode::Add {
                offset: -2,
                value: 10,
            },
        ];

        let result = defer_movements(ir);

        assert_eq!(result, expected);
    }
}
