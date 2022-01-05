use crate::interpreter::InterpreterEvents;
use crate::parser::ast::Node;

#[cfg(not(target_arch = "wasm32"))]
pub struct DoubleAgent {
    socket: zmq::Socket,
}

#[cfg(not(target_arch = "wasm32"))]
impl DoubleAgent {
    pub fn new() -> DoubleAgent {
        let ctx = zmq::Context::new();
        let socket = ctx.socket(zmq::PAIR).unwrap();
        socket.connect("tcp://localhost:5634").unwrap();

        DoubleAgent { socket }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl InterpreterEvents for DoubleAgent {
    fn on_instruction(&self, instruction: &Node) {
        let description = match &instruction {
            &Node::Backward => "<  Backward".to_string(),
            &Node::Forward => ">  Forward".to_string(),
            &Node::Increment => "+  Increment".to_string(),
            &Node::Decrement => "-  Decrement".to_string(),
            &Node::Input => ",  Input".to_string(),
            &Node::Output => ".  Output".to_string(),
            &Node::Loop { children } => format!("[] Loop with {} children", children.len()),
        };

        self.socket.send(description.as_str(), 0).unwrap();
    }

    fn on_start(&self) {
        self.socket.send("Execution started", 0).unwrap();
    }

    fn on_complete(&self) {
        self.socket.send("Execution completed", 0).unwrap();
    }
}
