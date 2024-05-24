enum TokenKind {
    Keyword(Keyword),
    Identifier(String),
    Operator(Operator),
    Symbol(Symbol),
}

enum Keyword {
    Function,
    Let,
}

enum Literal {
    Integer(i32),
    // Float(f32),
    // String(String),
}

enum Operator {
    Plus,
    // Minus,
    // Multiply,
    // Divide,
}

enum Symbol {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Bar,
    Semicolon,
    Colon,
    Comma,
    Equal,
}

struct Token {
    kind: TokenKind,
    row: usize,
    col: usize,
}

struct Tokenizer<'a> {
    row: usize,
    col: usize,
    remaining_text: &'a str,
}

impl<'a> Tokenizer<'a> {
    fn new(value: &str) -> Tokenizer {
        Tokenizer {
            row: 0,
            col: 0,
            remaining_text: value,
        }
    }
}

pub fn tokenize(src: &str) -> Result<Vec<Token>> {}
