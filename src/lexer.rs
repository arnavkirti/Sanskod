use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let ch = self.input[self.position];
            self.position += 1;
            Some(ch)
        } else {
            None
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(ch) = self.next_char() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => continue, // Ignore whitespace
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Multiply),
                '/' => tokens.push(Token::Divide),
                '=' => {
                    if let Some(next_ch) = self.next_char() {
                        if next_ch == '=' {
                            tokens.push(Token::Equals);
                        } else {
                            self.position -= 1;
                            tokens.push(Token::Assign);
                        }
                    } else {
                        tokens.push(Token::Assign);
                    }
                },
                '(' => tokens.push(Token::LeftParen),
                ')' => tokens.push(Token::RightParen),
                '{' => tokens.push(Token::LeftBrace),
                '}' => tokens.push(Token::RightBrace),
                '"' => {
                    let mut string = String::new();
                    while let Some(next_ch) = self.next_char() {
                        if next_ch == '"' { break; }
                        string.push(next_ch);
                    }
                    tokens.push(Token::StringLiteral(string));
                }
                '0'..='9' => {
                    let mut number = ch.to_string();
                    while let Some(next_ch) = self.next_char() {
                        if next_ch.is_digit(10) {
                            number.push(next_ch);
                        } else {
                            self.position -= 1;
                            break;
                        }
                    }
                    tokens.push(Token::Number(number.parse().unwrap()));
                }
                _ => {
                    let mut identifier = ch.to_string();
                    while let Some(next_ch) = self.next_char() {
                        if next_ch.is_alphabetic() {
                            identifier.push(next_ch);
                        } else {
                            self.position -= 1;
                            break;
                        }
                    }
                    match identifier.as_str() {
                        "yadi" => tokens.push(Token::If),
                        "anyatha" => tokens.push(Token::Else),
                        "yavat" => tokens.push(Token::While),
                        "kriya" => tokens.push(Token::Function),
                        "pratyavartanam" => tokens.push(Token::Return),
                        "chapyati" => tokens.push(Token::Print),
                        _ => tokens.push(Token::Identifier(identifier)),
                    }
                }
            }
        }
        tokens.push(Token::EOF);
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_print() {
        let mut lexer = Lexer::new("chapyati(\"Hello\")");
        let tokens = lexer.tokenize();
        assert_eq!(tokens, vec![
            Token::Print,
            Token::LeftParen,
            Token::StringLiteral("Hello".to_string()),
            Token::RightParen,
            Token::EOF
        ]);
    }

    #[test]
    fn test_lexer_number() {
        let mut lexer = Lexer::new("42");
        let tokens = lexer.tokenize();
        assert_eq!(tokens, vec![
            Token::Number(42.0),
            Token::EOF
        ]);
    }

    #[test]
    fn test_lexer_whitespace() {
        let mut lexer = Lexer::new("   chapyati   (  \"Test\"  )   ");
        let tokens = lexer.tokenize();
        assert_eq!(tokens, vec![
            Token::Print,
            Token::LeftParen,
            Token::StringLiteral("Test".to_string()),
            Token::RightParen,
            Token::EOF
        ]);
    }
}