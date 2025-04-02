use crate::parser::ASTNode;

pub struct Interpreter;

impl Interpreter {
    pub fn run(node: &ASTNode) {
        match node {
            ASTNode::Print(text) => println!("{}", text),
            ASTNode::Number(value) => println!("{}", value),
            _ => println!("Unknown command"),
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
        Interpreter::run(&node);
    }

    #[test]
    fn test_interpret_number() {
        let node = ASTNode::Number(42);
        // Note: This test will actually print to stdout
        Interpreter::run(&node);
    }
}
