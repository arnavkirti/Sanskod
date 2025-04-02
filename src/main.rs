mod token;
mod lexer;

use lexer::Lexer;

fn main() {
    let input = "यदि (सत्य) { छपयति(\"नमस्ते\"); }";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    println!("{:?}", tokens);
}
