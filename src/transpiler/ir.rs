#[derive(Debug, Clone)]
pub enum IRNode {
    ForwardBackward(i32),
    IncrementDecrement(i32),
    Output,
    Input,
    Loop { children: Vec<IRNode> },
}
