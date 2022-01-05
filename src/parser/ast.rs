use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Node {
    Forward,
    Backward,
    Increment,
    Decrement,
    Output,
    Input,
    Loop { children: Vec<Node> },
}
