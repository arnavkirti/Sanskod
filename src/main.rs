mod token;
mod lexer;
mod parser;
mod interpreter;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::io::{self, Write};

fn main() {
    println!("Sanskrit Programming Language REPL Started!");
    loop {
        print!("Sanskod> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        // Create lexer and get tokens
        let mut lexer = Lexer::new(&input);
        let tokens = lexer.tokenize();

        // Create parser and get AST
        let mut parser = Parser::new(tokens);
        if let Some(ast) = parser.parse() {
            // Execute the AST using the interpreter
            Interpreter::run(&ast);
        }
    }
}
