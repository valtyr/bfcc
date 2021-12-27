#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IRNode {
    Move(i32),
    Add { value: i32, offset: i32 },
    Zero { offset: i32 },
    Output { offset: i32 },
    Input { offset: i32 },
    Loop { children: Vec<IRNode> },
}
