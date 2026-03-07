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

    /// Returns the next character without advancing pos.
    fn peek(&self) -> Option<char> {
        self.input.get(self.pos + 1).copied()
    }

    /// Reads one token from the current position and advances pos.
    fn next_token(&mut self) -> Token {
        // Skip whitespace
        while self.pos < self.input.len() && matches!(self.input[self.pos], ' ' | '\t' | '\n') {
            self.pos += 1;
        }

        if self.pos >= self.input.len() {
            return Token::Eof;
        }

        let ch = self.input[self.pos];

        match ch {
            'a'..='z' | 'A'..='Z' | '_' => {
                let word = self.read_while(|c| c.is_alphanumeric() || c == '_');
                match word.as_str() {
                    "int" => Token::Int,
                    "for" => Token::For,
                    _ => Token::Identifier(word),
                }
            }
            '0'..='9' => {
                let num = self.read_while(|c| c.is_numeric());
                Token::IntLiteral(num.parse().unwrap())
            }
            '=' => {
                if self.peek() == Some('=') { self.pos += 2; Token::EqualEqual }
                else { self.pos += 1; Token::Equals }
            }
            '!' => {
                if self.peek() == Some('=') { self.pos += 2; Token::NotEqual }
                else { self.pos += 1; Token::Eof }
            }
            '<' => {
                if self.peek() == Some('=') { self.pos += 2; Token::LessEqual }
                else { self.pos += 1; Token::Less }
            }
            '>' => {
                if self.peek() == Some('=') { self.pos += 2; Token::GreaterEqual }
                else { self.pos += 1; Token::Greater }
            }
            '+' => {
                if self.peek() == Some('=') { self.pos += 2; Token::PlusEquals }
                else { self.pos += 1; Token::Plus }
            }

            '-' => { 
                if self.peek() == Some('=') { self.pos += 2; Token::MinusEquals }
                else { self.pos += 1; Token::Minus }}
            ';' => { self.pos += 1; Token::Semicolon }
            '(' => { self.pos += 1; Token::LParen }
            ')' => { self.pos += 1; Token::RParen }
            '[' => { self.pos += 1; Token::LBracket }
            ']' => { self.pos += 1; Token::RBracket }
            '{' => { self.pos += 1; Token::LBrace }
            '}' => { self.pos += 1; Token::RBrace }
            ',' => { self.pos += 1; Token::Comma }
            
            '*' => { self.pos += 1; Token::Star }
            '/' => { self.pos += 1; Token::Slash }
            _ => { self.pos += 1; Token::Eof }
        }
    }

    /// Scans the entire input and returns a list of tokens.
    // v2: Accounts for multi-char operators ==, !=, <=, >=, +=
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            let is_eof = token == Token::Eof;
            tokens.push(token);
            if is_eof { break; }
        }
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

    #[test]
    fn test_not_equal() {
        let tokens = Lexer::new("i != 0").tokenize();
        assert_eq!(tokens, vec![
            Token::Identifier("i".to_string()),
            Token::NotEqual,
            Token::IntLiteral(0),
            Token::Eof,
        ]);
    }

    #[test]
    fn test_equal_equal() {
        let tokens = Lexer::new("i == 0").tokenize();
        assert_eq!(tokens, vec![
            Token::Identifier("i".to_string()),
            Token::EqualEqual,
            Token::IntLiteral(0),
            Token::Eof,
        ]);
    }

    #[test]
    fn test_less_equal() {
        let tokens = Lexer::new("i <= 8").tokenize();
        assert_eq!(tokens, vec![
            Token::Identifier("i".to_string()),
            Token::LessEqual,
            Token::IntLiteral(8),
            Token::Eof,
        ]);
    }

    #[test]
    fn test_plus_equals() {
        let tokens = Lexer::new("sum += 1").tokenize();
        assert_eq!(tokens, vec![
            Token::Identifier("sum".to_string()),
            Token::PlusEquals,
            Token::IntLiteral(1),
            Token::Eof,
        ]);
    }


    #[test]
    fn test_minus_equals() {
        let tokens = Lexer::new("minus -= 3").tokenize();
        assert_eq!(tokens, vec![
            Token::Identifier("minus".to_string()),
            Token::MinusEquals,
            Token::IntLiteral(3),
            Token::Eof,
        ]);
    }



}
