#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Identifier(String),

    // Keywords
    Function,
    Let,

    // Literals
    Integer(i32),
    Float(f32),
    // String(String),

    // Operators
    Plus,
    // Minus,
    // Multiply,
    // Divide,

    // Symbols
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Bar,
    Semicolon,
    Colon,
    Comma,
    Dot,
    Equal,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub row: usize,
    pub col: usize,
}
