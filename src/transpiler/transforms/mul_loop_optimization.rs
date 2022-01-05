use crate::transpiler::ir::IRNode;
use rustc_hash::FxHashMap;

fn optimize_children(children: &Vec<IRNode>) -> Option<Vec<IRNode>> {
    let mut transformed: Vec<IRNode> = vec![];
    let mut accumulated_offset = 0;

    // Map of offset to change
    let mut deltas: FxHashMap<i32, i32> = FxHashMap::default();

    for node in children {
        match node {
            IRNode::Add { offset, value } => {
                let key = accumulated_offset + offset;
                let previous = deltas.get(&key);
                let new = *(previous.unwrap_or(&0)) + value;
                deltas.insert(key, new);
            }
            IRNode::Move(offset) => {
                accumulated_offset += offset;
            }
            _ => {
                // If we reach any other nodes we bail
                return None;
            }
        }
    }

    // If the delta at offset 0 isn't -1 we bail
    if deltas.get(&0) != Some(&(-1)) {
        return None;
    }

    // If the accumulated offset isn't 0 at the end of the loop we bail
    if accumulated_offset != 0 {
        return None;
    }

    for pair in deltas {
        match pair {
            (0, _) => transformed.push(IRNode::Zero { offset: 0 }),
            (offset, value) => {
                // We don't know how to handle negative multiplications
                if value < 0 {
                    return None;
                }
                transformed.push(IRNode::Mul { offset, value })
            }
        }
    }

    Some(transformed)
}

pub fn mul_loop_optimization(branch: Vec<IRNode>) -> Vec<IRNode> {
    let mut transformed: Vec<IRNode> = vec![];
    for node in branch {
        match node {
            IRNode::Loop { ref children } => {
                let optimized = optimize_children(children);
                match optimized {
                    Some(children) => transformed.push(IRNode::Loop { children }),
                    _ => transformed.push(node),
                }
            }
            _ => transformed.push(node),
        }
    }

    transformed
}

#[cfg(test)]
mod tests {
    use crate::transpiler::ir::IRNode;
    use pretty_assertions::assert_eq;

    use super::optimize_children;

    #[test]
    fn test_single_mul() {
        let children = vec![
            IRNode::Add {
                offset: 0,
                value: -1,
            },
            IRNode::Move(1),
            IRNode::Add {
                offset: 0,
                value: 3,
            },
            IRNode::Move(-1),
        ];
        let results = optimize_children(&children);

        assert_eq!(
            results.unwrap(),
            vec![
                IRNode::Zero { offset: 0 },
                IRNode::Mul {
                    offset: 1,
                    value: 3
                }
            ]
        );
    }

    #[test]
    fn test_double_mul() {
        let children = vec![
            IRNode::Add {
                offset: 0,
                value: -1,
            },
            IRNode::Move(1),
            IRNode::Add {
                offset: 0,
                value: 5,
            },
            IRNode::Move(1),
            IRNode::Add {
                offset: 0,
                value: 2,
            },
            IRNode::Move(-2),
        ];
        let results = optimize_children(&children);

        assert_eq!(
            results.unwrap(),
            vec![
                IRNode::Zero { offset: 0 },
                IRNode::Mul {
                    offset: 1,
                    value: 5
                },
                IRNode::Mul {
                    offset: 2,
                    value: 2
                }
            ]
        );
    }

    #[test]
    fn test_copy() {
        let children = vec![
            IRNode::Add {
                offset: 0,
                value: -1,
            },
            IRNode::Add {
                offset: 1,
                value: 1,
            },
            IRNode::Add {
                offset: 2,
                value: 1,
            },
        ];
        let results = optimize_children(&children);

        assert_eq!(
            results.unwrap(),
            vec![
                IRNode::Zero { offset: 0 },
                IRNode::Mul {
                    offset: 1,
                    value: 1
                },
                IRNode::Mul {
                    offset: 2,
                    value: 1
                }
            ]
        );
    }

    #[test]
    fn test_bail_on_non_add_or_move() {
        let children = vec![IRNode::Zero { offset: 0 }];
        let results = optimize_children(&children);

        assert_eq!(results, None);
    }

    #[test]
    fn test_bail_on_non_zero_accumulated_offset() {
        let children = vec![
            IRNode::Add {
                offset: 0,
                value: -1,
            },
            IRNode::Move(1),
        ];
        let results = optimize_children(&children);

        assert_eq!(results, None);
    }

    #[test]
    fn test_bail_on_pos_zero_not_minus_one() {
        let children = vec![
            IRNode::Add {
                offset: 0,
                value: -2,
            },
            IRNode::Add {
                offset: 1,
                value: 2,
            },
        ];
        let results = optimize_children(&children);

        assert_eq!(results, None);
    }

    #[test]
    fn test_bail_on_negative_mul() {
        let children = vec![
            IRNode::Add {
                offset: 0,
                value: -1,
            },
            IRNode::Add {
                offset: 1,
                value: -2,
            },
        ];
        let results = optimize_children(&children);

        assert_eq!(results, None);
    }
}
