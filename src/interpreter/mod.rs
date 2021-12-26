use crate::parser::ast::Node;
use crate::utils;

pub struct Interpreter {
    pointer: u16,
    memory: [u8; 30000],
    hooks: Vec<Box<dyn InterpreterEvents>>,
}

pub trait InterpreterEvents {
    fn on_start(&self) {}
    fn on_instruction(&self, _instruction: &Node) {}
    fn on_complete(&self) {}
}

impl Interpreter {
    pub fn run(&mut self, ast: &Vec<Node>) {
        for hook in &self.hooks {
            hook.on_start();
        }

        for node in ast {
            self.execute_instruction(node)
        }

        for hook in &self.hooks {
            hook.on_complete();
        }
    }

    fn execute_instruction(&mut self, node: &Node) {
        for hook in &self.hooks {
            hook.on_instruction(&node);
        }

        match node {
            Node::Forward => self.pointer += 1,
            Node::Backward => self.pointer -= 1,
            Node::Decrement => {
                self.memory[self.pointer as usize] =
                    utils::overflow_add(self.memory[self.pointer as usize], 255)
            }
            Node::Increment => {
                self.memory[self.pointer as usize] =
                    utils::overflow_add(self.memory[self.pointer as usize], 1)
            }
            Node::Input => match utils::read_byte() {
                Some(byte) => self.memory[self.pointer as usize] = byte,
                None => {}
            },
            Node::Output => print!("{}", self.memory[self.pointer as usize] as char),
            Node::Loop { children } => {
                while self.memory[self.pointer as usize] != 0 {
                    for child in children {
                        self.execute_instruction(child)
                    }
                }
            }
        }
    }

    pub fn add_events_hook<E: InterpreterEvents + 'static>(&mut self, hook: E) {
        self.hooks.push(Box::new(hook));
    }
}

pub fn new() -> Interpreter {
    Interpreter {
        pointer: 0,
        memory: [0; 30000],
        hooks: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::new;
    use crate::parser::ast::Node;

    #[test]
    fn test_increment() {
        let program = vec![Node::Increment];
        let mut interpreter = new();
        interpreter.run(&program);
        assert!(interpreter.memory[0] == 1);
    }

    #[test]
    fn test_decrement() {
        let program = vec![Node::Decrement];
        let mut interpreter = new();

        interpreter.memory[0] = 1;

        interpreter.run(&program);
        assert!(interpreter.memory[0] == 0);
    }

    #[test]
    fn test_forward() {
        let program = vec![Node::Forward];
        let mut interpreter = new();
        interpreter.run(&program);
        assert!(interpreter.pointer == 1);
    }

    #[test]
    fn test_backward() {
        let program = vec![Node::Backward];

        let mut interpreter = new();
        interpreter.pointer = 1;

        interpreter.run(&program);
        assert!(interpreter.pointer == 0);
    }

    #[test]
    fn test_loop() {
        let program = vec![
            Node::Loop {
                children: vec![Node::Increment],
            },
            Node::Forward,
            Node::Decrement,
        ];

        let mut interpreter = new();

        interpreter.run(&program);
        assert_eq!(interpreter.pointer, 1);
        assert_eq!(interpreter.memory[0], 0);
        assert_eq!(interpreter.memory[1], 255);
    }
}
