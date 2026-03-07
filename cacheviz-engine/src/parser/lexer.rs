use crate::parser::token::Token;

/// Converts raw source code into a flat list of tokens.
pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    /// Creates a new Lexer from the given source string.
    pub fn new(source: &str) -> Self {
        Lexer {
            input: source.chars().collect(),
            pos: 0,
        }
    }

    /// Advances pos while predicate is true, returns the collected characters as a String.
    fn read_while(&mut self, predicate: impl Fn(char) -> bool) -> String {
        let start = self.pos;
        while self.pos < self.input.len() && predicate(self.input[self.pos]) {
            self.pos += 1;
        }
        self.input[start..self.pos].iter().collect()
    }

    /// Scans the input and returns a list of tokens.
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.pos < self.input.len() {
            let ch = self.input[self.pos];

            match ch {
                'a'..='z' | 'A'..='Z' | '_' => {
                    let word = self.read_while(|c| c.is_alphanumeric() || c == '_');
                    let token = match word.as_str() {
                        "int" => Token::Int,
                        "for" => Token::For,
                        _ => Token::Identifier(word),
                    };
                    tokens.push(token);
                }
                '0'..='9' => {
                    let num = self.read_while(|c| c.is_numeric());
                    tokens.push(Token::IntLiteral(num.parse().unwrap()));
                }
                '=' => { tokens.push(Token::Equals); self.pos += 1; }
                ';' => { tokens.push(Token::Semicolon); self.pos += 1; }
                '(' => { tokens.push(Token::LParen); self.pos += 1; }
                ')' => { tokens.push(Token::RParen); self.pos += 1; }
                '[' => { tokens.push(Token::LBracket); self.pos += 1; }
                ']' => { tokens.push(Token::RBracket); self.pos += 1; }
                '{' => { tokens.push(Token::LBrace); self.pos += 1; }
                '}' => { tokens.push(Token::RBrace); self.pos += 1; }
                ',' => { tokens.push(Token::Comma); self.pos += 1; }
                '+' => { tokens.push(Token::Plus); self.pos += 1; }
                '-' => { tokens.push(Token::Minus); self.pos += 1; }
                '*' => { tokens.push(Token::Star); self.pos += 1; }
                '/' => { tokens.push(Token::Slash); self.pos += 1; }
                '<' => { tokens.push(Token::Less); self.pos += 1; }
                '>' => { tokens.push(Token::Greater); self.pos += 1; }
                ' ' | '\t' | '\n' => { self.pos += 1; }
                _ => { self.pos += 1; }
            }
        }

        tokens.push(Token::Eof);
        tokens
    }
}


// Tests for lexer.rs, run 'cargo test parser' in terminal
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_declaration() {
        let tokens = Lexer::new("int x = 5;").tokenize();
        assert_eq!(tokens, vec![
            Token::Int,
            Token::Identifier("x".to_string()),
            Token::Equals,
            Token::IntLiteral(5),
            Token::Semicolon,
            Token::Eof,
        ]);
    }

    #[test]
    fn test_array_declaration() {
        let tokens = Lexer::new("int A[8];").tokenize();
        assert_eq!(tokens, vec![
            Token::Int,
            Token::Identifier("A".to_string()),
            Token::LBracket,
            Token::IntLiteral(8),
            Token::RBracket,
            Token::Semicolon,
            Token::Eof,
        ]);
    }

    #[test]
    fn test_for_loop() {
        let tokens = Lexer::new("for (int i = 0; i < 8; i++)").tokenize();
        assert_eq!(tokens, vec![
            Token::For,
            Token::LParen,
            Token::Int,
            Token::Identifier("i".to_string()),
            Token::Equals,
            Token::IntLiteral(0),
            Token::Semicolon,
            Token::Identifier("i".to_string()),
            Token::Less,
            Token::IntLiteral(8),
            Token::Semicolon,
            Token::Identifier("i".to_string()),
            Token::Plus,
            Token::Plus,
            Token::RParen,
            Token::Eof,
        ]);
    }

    #[test]
    fn test_source_code1() {
        let tokens = Lexer::new("int A[8]; for (int i = 0; i < 8; i++) { A[i] = i; }").tokenize();
        assert_eq!(tokens, vec![
            Token::Int,
            Token::Identifier("A".to_string()),
            Token::LBracket,
            Token::IntLiteral(8),
            Token::RBracket,
            Token::Semicolon,
            Token::For,
            Token::LParen,
            Token::Int,
            Token::Identifier("i".to_string()),
            Token::Equals,
            Token::IntLiteral(0),
            Token::Semicolon,
            Token::Identifier("i".to_string()),
            Token::Less,
            Token::IntLiteral(8),
            Token::Semicolon,
            Token::Identifier("i".to_string()),
            Token::Plus,
            Token::Plus,
            Token::RParen,
            Token::LBrace,
            Token::Identifier("A".to_string()),
            Token::LBracket,
            Token::Identifier("i".to_string()),
            Token::RBracket,
            Token::Equals,
            Token::Identifier("i".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Eof,
        ]);
    }

    
}
