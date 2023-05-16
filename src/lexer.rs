use crate::logging;

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Fn,
    Let
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

#[derive(Debug)]
pub struct Loc {
    line: usize,
    col: usize
}

impl std::fmt::Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.col)
    }
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub loc: Loc
}

pub struct Lexer {
    cursor: usize,
    line_n: usize,
    bol: usize,
    pub file: String,
    chars: Vec<char>,
    is_exhausted: bool,
}

fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

impl Lexer {
    pub fn new(filepath: &str) -> Self {
        let contents = std::fs::read_to_string(filepath).unwrap_or_else(|e| logging::io_err(e));
        Self {
            chars: contents.chars().collect(),
            file: filepath.to_owned(),
            cursor: 0,
            line_n: 1,
            bol: 0,
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
                if c == '\n' {
                    self.bol = self.cursor;
                    self.line_n += 1;
                }
                if !c.is_whitespace() {
                    self.cursor -= 1;
                    break;
                }
            }
        }
    }

    pub fn loc(&self) -> Loc {
        Loc {
            line: self.line_n,
            col: self.cursor - self.bol
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
            x if is_ident_char(x) => {
                let mut text = String::new();
                text.push(x);
                while let Some(x) = self.chop_char_if(|x| is_ident_char(x)) {
                    text.push(x);
                }
                match &text[..] {
                    "fn" => Kword(Keyword::Fn),
                    "let" => Kword(Keyword::Let),
                    _ => Identifier(text)
                }
            }
            other => logging::syntax_err(&self.file, self.loc(), &format!("Invalid token starts with `{}`", other))
        };
        Some(Token {kind, loc: self.loc()})
    }

    pub fn expect(&mut self, what: TokenKind) -> Token {
        if let Some(tok) = self.next() {
            if tok.kind == what {
                tok
            } else {
                logging::unexpected_token(&self.file, what, tok);
            }
        } else {
            logging::no_expected_token(&self.file, what);
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}
