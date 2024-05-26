use crate::errors::{ErrorKind, Result};
use crate::extensions::iter::IteratorExt;
use crate::lexer::tokens::{Token, TokenKind};

impl Token {
    fn new(kind: TokenKind, tokenizer: &Tokenizer) -> Self {
        Token {
            kind,
            row: tokenizer.row,
            col: tokenizer.col,
        }
    }
}

fn tokenize_number(data: &str) -> Result<(TokenKind, usize)> {
    let mut seen_dot = false;
    let bytes_read = data
        .chars()
        .limit_count(|&c| c == '.', 1)
        .take_while(|&c| {
            if c == '.' {
                if seen_dot {
                    return false;
                }
                seen_dot = true;
                return true;
            }
            c.is_ascii_digit() || c == '_'
        })
        .count();
    let matched = &data[..bytes_read].replace('_', "");
    if seen_dot {
        Ok((TokenKind::Float(matched.parse()?), matched.len()))
    } else {
        Ok((TokenKind::Integer(matched.parse()?), matched.len()))
    }
}

fn read_identifier_like(data: &str) -> (&str, usize) {
    let bytes_read = data
        .chars()
        .take_while(|&c| c.is_alphanumeric() || c == '_')
        .count();
    let matched = &data[..bytes_read];
    (matched, bytes_read)
}

struct Tokenizer<'a> {
    row: usize,
    col: usize,
    remaining_text: &'a str,
}

impl<'a> From<&'a str> for Tokenizer<'a> {
    fn from(value: &'a str) -> Self {
        Tokenizer {
            row: 0,
            col: 0,
            remaining_text: value,
        }
    }
}

impl<'a> Tokenizer<'a> {
    fn skip(&mut self) {
        loop {
            let mut data = self.remaining_text;
            // Skip whitespace
            let skipped = data.chars().take_while(|c| {
                if c.is_whitespace() {
                    match c {
                        '\n' => {
                            self.row += 1;
                            self.col = 0;
                        }
                        '\t' => self.col += 4,
                        _ => self.col += 1,
                    }
                    true
                } else {
                    false
                }
            });
            data = &data[skipped.count()..];

            // Skip comments
            let pairs = [("//", "\n"), ("/*", "*/")];
            for (start, end) in pairs {
                if data.starts_with(start) {
                    while !data.is_empty() && !data.starts_with(end) {
                        let next_size = data.chars().next().expect("String isn't empty").len_utf8();
                        data = &data[next_size..];
                    }
                    data = &data[end.len()..];
                }
            }

            if data.len() == self.remaining_text.len() {
                break;
            }
            self.remaining_text = data;
        }
    }

    fn next(&mut self) -> Result<Option<Token>> {
        self.skip();

        if self.remaining_text.is_empty() {
            return Ok(None);
        };

        let next = self.remaining_text.chars().next().unwrap();

        let (kind, size) = match next {
            // Symbols
            '(' => (TokenKind::OpenParen, 1),
            ')' => (TokenKind::CloseParen, 1),
            '{' => (TokenKind::OpenBrace, 1),
            '}' => (TokenKind::CloseBrace, 1),
            '|' => (TokenKind::Bar, 1),
            ';' => (TokenKind::Semicolon, 1),
            ':' => (TokenKind::Colon, 1),
            ',' => (TokenKind::Comma, 1),
            '.' => (TokenKind::Dot, 1),
            '=' => (TokenKind::Equal, 1),

            // Operators
            '+' => (TokenKind::Plus, 1),

            // Other
            '0'..='9' => tokenize_number(self.remaining_text)?,
            c @ '_' | c if c.is_alphanumeric() => {
                let (id, length) = read_identifier_like(self.remaining_text);
                let kind = match id {
                    "fn" => TokenKind::Function,
                    "let" => TokenKind::Let,
                    _ => TokenKind::Identifier(id.into()),
                };
                (kind, length)
            }
            _ => return Err(ErrorKind::UnknownCharacter(next).into()),
        };

        let token = Token::new(kind, self);
        self.advance(size);
        Ok(Some(token))
    }

    fn advance(&mut self, amount: usize) {
        self.remaining_text = &self.remaining_text[amount..];
        self.col += amount;
    }
}

pub fn tokenize(src: &str) -> Result<Vec<Token>> {
    let mut tokenizer = Tokenizer::from(src);
    let mut tokens = Vec::new();

    while let Some(token) = tokenizer.next()? {
        tokens.push(token);
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_next_token {
        ($name:ident, $src:expr => $should_be:expr) => {
            #[test]
            fn $name() {
                let src: &str = $src;
                let should_be = $should_be;

                let mut tokenizer = Tokenizer::from(src);
                match tokenizer.next() {
                    Ok(Some(token)) => assert_eq!(token.kind, should_be, "Got {:?}", should_be),
                    _ => panic!("Tokenizing got error"),
                }
            }
        };
    }

    macro_rules! test_tokenize {
        ($name:ident, $src:expr => $should_be:expr) => {
            #[test]
            fn $name() {
                let src: &str = $src;
                let should_be = $should_be;

                let tokens = tokenize(src).unwrap();
                assert_eq!(tokens, should_be);
            }
        };
    }

    test_next_token!(tokenize_open_brace, "{" => TokenKind::OpenBrace);
    test_next_token!(tokenize_open_brace_with_trail, "}fdsfs" => TokenKind::CloseBrace);
    test_next_token!(tokenize_function_keyword, "fn" => TokenKind::Function);
    test_next_token!(tokenize_let_keyword, "let" => TokenKind::Let);

    test_next_token!(tokenize_single_digit, "1" => TokenKind::Integer(1));
    test_next_token!(tokenize_multiple_digits, "1234567890" => TokenKind::Integer(1_234_567_890));
    test_next_token!(tokenize_integer_with_underscore, "1_000" => TokenKind::Integer(1000));
    test_next_token!(tokenize_basic_float, "1.2" => TokenKind::Float(1.2));
    test_next_token!(tokenize_float_with_underscore, "1_000.2_000" => TokenKind::Float(1000.2));
    test_next_token!(tokenize_float_with_underscore_and_trailing, "1_000.2_000_000" => TokenKind::Float(1000.2));
    test_next_token!(tokenize_float_without_decimal, "1." => TokenKind::Float(1.0));

    test_tokenize!(
        tokenize_plus_expression,
        "2 + 5.5" => vec![
            Token {
                kind: TokenKind::Integer(2),
                row: 0,
                col: 0,
            },
            Token {
                kind: TokenKind::Plus,
                row: 0,
                col: 2,
            },
            Token {
                kind: TokenKind::Float(5.5),
                row: 0,
                col: 4,
            },
        ]
    );

    test_tokenize!(
        tokenize_function_declaration,
        "fn first(a: int, b: int) { a }" => vec![
            Token {
                kind: TokenKind::Function,
                row: 0,
                col: 0,
            },
            Token {
                kind: TokenKind::Identifier("first".into()),
                row: 0,
                col: 3,
            },
            Token {
                kind: TokenKind::OpenParen,
                row: 0,
                col: 8,
            },
            Token {
                kind: TokenKind::Identifier("a".into()),
                row: 0,
                col: 9,
            },
            Token {
                kind: TokenKind::Colon,
                row: 0,
                col: 10,
            },
            Token {
                kind: TokenKind::Identifier("int".into()),
                row: 0,
                col: 12,
            },
            Token {
                kind: TokenKind::Comma,
                row: 0,
                col: 15,
            },
            Token {
                kind: TokenKind::Identifier("b".into()),
                row: 0,
                col: 17,
            },
            Token {
                kind: TokenKind::Colon,
                row: 0,
                col: 18,
            },
            Token {
                kind: TokenKind::Identifier("int".into()),
                row: 0,
                col: 20,
            },
            Token {
                kind: TokenKind::CloseParen,
                row: 0,
                col: 23,
            },
            Token {
                kind: TokenKind::OpenBrace,
                row: 0,
                col: 25,
            },
            Token {
                kind: TokenKind::Identifier("a".into()),
                row: 0,
                col: 27,
            },
            Token {
                kind: TokenKind::CloseBrace,
                row: 0,
                col: 29,
            },
        ]
    );
}
