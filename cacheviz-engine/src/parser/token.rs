

// Tokens for C-like language
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literals
    IntLiteral(i64),
    Identifier(String),

    // Keywords
    Int,
    For,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Equals,        // =
    Less,          // <
    LessEqual,     // <=
    Greater,       // >
    GreaterEqual,  // >=
    PlusEquals,    // +=

    // Delimiters
    LParen,    // (
    RParen,    // )
    LBracket,  // [
    RBracket,  // ]
    LBrace,    // {
    RBrace,    // }
    Semicolon,
    Comma,

    // Special
    Eof,
}
