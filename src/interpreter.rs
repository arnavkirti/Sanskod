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
