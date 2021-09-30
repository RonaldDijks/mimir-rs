use std::{iter::Peekable, str::CharIndices};

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Integer,
    Plus,
    Error,
    EndOfFile,
}

#[derive(Debug)]
pub struct Token {
    /// The kind of token
    pub kind: TokenKind,

    /// The text that produced this token
    pub lexeme: String,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: &str) -> Token {
        Token {
            kind,
            lexeme: lexeme.to_string(),
        }
    }
}

/// A scanner for the language
pub struct Lexer<'a> {
    /// The input source
    source: &'a str,

    /// A peekable char indices iterator
    char_indices: Peekable<CharIndices<'a>>,

    /// The start of the current token
    current_index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let char_indices = source.char_indices().peekable();

        Self {
            source,
            char_indices,
            current_index: 0,
        }
    }

    fn next(&mut self) -> Option<char> {
        self.char_indices.next().map(|(index, char)| {
            self.current_index = index;
            char
        })
    }

    fn peek(&mut self) -> Option<char> {
        self.char_indices.peek().map(|(_, c)| *c)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.next();
            } else {
                return;
            }
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.next() {
            None => Token::new(TokenKind::EndOfFile, ""),
            Some(c) => match c {
                '+' => Token::new(TokenKind::Plus, "+"),
                x => {
                    if x.is_numeric() {
                        self.scan_number()
                    } else {
                        Token::new(TokenKind::Error, "error")
                    }
                }
            },
        }
    }

    fn scan_number(&mut self) -> Token {
        let start = self.current_index;

        while let Some(x) = self.peek() {
            if x.is_numeric() {
                self.next();
            } else {
                break;
            }
        }

        let kind = TokenKind::Integer;
        let lexeme = &self.source[start..=self.current_index];
        Token::new(kind, lexeme)
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, TokenKind};

    #[test]
    fn lexes_binary_expression() {
        let source = "1 + 2";
        let mut lexer = Lexer::new(source);

        let token = lexer.scan_token();
        assert_eq!(token.kind, TokenKind::Integer);
        assert_eq!(token.lexeme, "1");

        let token = lexer.scan_token();
        assert_eq!(token.kind, TokenKind::Plus);
        assert_eq!(token.lexeme, "+");

        let token = lexer.scan_token();
        assert_eq!(token.kind, TokenKind::Integer);
        assert_eq!(token.lexeme, "2");
    }
}
