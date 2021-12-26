#[derive(Debug, Clone)]
pub enum Node {
    Forward,
    Backward,
    Increment,
    Decrement,
    Output,
    Input,
    Loop { children: Vec<Node> },
}
