#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String), // variable name, function name
    Number(f64), // number values
    StringLiteral(String), // string values

    // Keywords
    If,        // यदि
    Else,      // अन्यथा
    While,     // यावत्
    Function,  // क्रिया
    Return,    // प्रत्यावर्तनम्
    Print,     // छपयति

    // Operators
    Plus,      // संयोग (+)
    Minus,     // वियोग (-)
    Multiply,  // गुणन (*)
    Divide,    // विभाजन (/)
    Assign,    // सम (=)
    Equals,    // समान (==)
    
    // Punctuation
    LeftParen,  // (
    RightParen, // )
    LeftBrace,  // {
    RightBrace, // }
    
    EOF,
}