use crate::parser::ASTNode;
use crate::token::Token;

use std::collections::HashMap;

pub struct Interpreter {
    variables: HashMap<String, Value>,
}

#[derive(Clone, Debug)]
pub enum Value {
    Number(i64),
    String(String),
    Boolean(bool),
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn run(&mut self, node: &ASTNode) -> Option<Value> {
        match node {
            ASTNode::Print(text) => {
                println!("{}", text);
                None
            }
            ASTNode::Number(n) => Some(Value::Number(*n)),
            ASTNode::VarDeclaration(name, expr) => {
                if let Some(value) = self.run(expr) {
                    self.variables.insert(name.clone(), value);
                }
                None
            }
            ASTNode::VarReference(name) => {
                self.variables.get(name).cloned()
            }
            ASTNode::BinaryOp(left, op, right) => {
                self.evaluate_binary_op(left, op, right)
            }
            _ => None,
        }
    }

    fn evaluate_binary_op(&mut self, left: &ASTNode, op: &Token, right: &ASTNode) -> Option<Value> {
        let left_val = self.run(left)?;
        let right_val = self.run(right)?;

        match (left_val, op, right_val) {
            (Value::Number(l), Token::Plus, Value::Number(r)) => Some(Value::Number(l + r)),
            (Value::Number(l), Token::Minus, Value::Number(r)) => Some(Value::Number(l - r)),
            (Value::Number(l), Token::Multiply, Value::Number(r)) => Some(Value::Number(l * r)),
            (Value::Number(l), Token::Divide, Value::Number(r)) => Some(Value::Number(l / r)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpret_print() {
        let node = ASTNode::Print("Hello".to_string());
        // Note: This test will actually print to stdout
        let mut interpreter = Interpreter::new();
        interpreter.run(&node);
    }

    #[test]
    fn test_interpret_number() {
        let node = ASTNode::Number(42);
        // Note: This test will actually print to stdout
        let mut interpreter = Interpreter::new();
        interpreter.run(&node);
    }
}
