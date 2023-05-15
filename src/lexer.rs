use crate::logging;

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Fn
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Str(String)
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    Kword(Keyword),
    Identifier(String),
    Litrl(Literal),
    Semicolon,
    Comma,
    Invalid,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
}

pub struct Lexer {
    cursor: usize,
    chars: Vec<char>,
    is_exhausted: bool,
}

impl Lexer {
    pub fn new(chars: Vec<char>) -> Self {
        Self {
            chars,
            cursor: 0,
            is_exhausted: false
        }
    }

    fn chop_char(&mut self) -> Option<char> {
        let c = self.chars.get(self.cursor).copied();
        if c.is_some() {
            self.cursor += 1;
        } else {
            self.is_exhausted = true;
        }
        c
    }
    fn chop_char_if(&mut self, condition: fn(char) -> bool) -> Option<char> {
        let c = self.chars.get(self.cursor).copied();
        if c.is_some() {
            if condition(c.unwrap()) {
                self.cursor += 1;
            }
            else {
                return None;
            }
        } else {
            self.is_exhausted = true;
        }
        c
    }

    fn trim_whitespace(&mut self) {
        while !self.is_exhausted {
            if let Some(c) = self.chop_char() {
                if !c.is_whitespace() {
                    self.cursor -= 1;
                    break;
                }
            }
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        use TokenKind::*;
        self.trim_whitespace();
        let c = self.chop_char()?;
        let kind = match c {
            x if x.is_ascii_digit() => {
                todo!("number literals");
            }
            '(' => OpenParen,
            ')' => CloseParen,
            '{' => OpenCurly,
            '}' => CloseCurly,
            ';' => Semicolon,
            ',' => Comma,
            '"' => {
                let mut text = String::new();
                while let Some(x) = self.chop_char() {
                    if x == '"' {
                        break;
                    }
                    if x == '\\' {
                        if let Some(escaped_char) = self.chop_char() {
                            match escaped_char {
                                'n' => {
                                    text.push('\n');
                                }
                                _ => todo!("Error reporting "),
                            }
                        } else {
                            todo!("Error reporting ");
                        }
                    } else {
                        text.push(x);
                    }
                }
                Litrl(Literal::Str(text))
            }
            x if x.is_alphabetic() => {
                let mut text = String::new();
                text.push(x);
                while let Some(x) = self.chop_char_if(|x| x.is_alphanumeric()) {
                    text.push(x);
                }
                match &text[..] {
                    "fn" => Kword(Keyword::Fn),
                    _ => Identifier(text)
                }
            }
            _ => Invalid
        };
        Some(Token {kind})
    }

    pub fn expect(&mut self, what: TokenKind) -> Token {
        if let Some(tok) = self.next() {
            if tok.kind == what {
                tok
            } else {
                logging::unexpected_token(tok.kind, what);
            }
        } else {
            logging::no_expected_token(what);
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}
