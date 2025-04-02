use crate::token::Token;

#[derive(Debug)]
pub enum ASTNode {
    Print(String),
    Number(i64),
    Variable(String, i64),
    BinaryOp(Box<ASTNode>, Token, Box<ASTNode>),
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    fn next_token(&mut self) -> &Token {
        &self.tokens[self.position]
    }

    fn eat(&mut self) {
        self.position += 1;
    }

    pub fn parse(&mut self) -> Option<ASTNode> {
        match self.next_token() {
            Token::Print => {
                self.eat(); // consume 'chapyati'
                match self.next_token() {
                    Token::LeftParen => {
                        self.eat(); // consume '('
                        if let Token::StringLiteral(value) = self.next_token().clone() {
                            self.eat(); // consume string
                            match self.next_token() {
                                Token::RightParen => {
                                    self.eat(); // consume ')'
                                    Some(ASTNode::Print(value))
                                }
                                _ => None
                            }
                        } else {
                            None
                        }
                    }
                    _ => None
                }
            }
            Token::Number(value) => {
                let num = *value;
                self.eat();
                Some(ASTNode::Number(num as i64))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_print() {
        let tokens = vec![
            Token::Print,
            Token::LeftParen,
            Token::StringLiteral("Hello".to_string()),
            Token::RightParen,
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        if let Some(ASTNode::Print(text)) = parser.parse() {
            assert_eq!(text, "Hello");
        } else {
            panic!("Failed to parse print statement");
        }
    }

    #[test]
    fn test_parse_number() {
        let tokens = vec![
            Token::Number(42.0),
            Token::EOF
        ];
        let mut parser = Parser::new(tokens);
        if let Some(ASTNode::Number(num)) = parser.parse() {
            assert_eq!(num, 42);
        } else {
            panic!("Failed to parse number");
        }
    }

    #[test]
    fn test_parse_invalid() {
        let tokens = vec![Token::EOF];
        let mut parser = Parser::new(tokens);
        assert!(parser.parse().is_none());
    }
}
