use super::*;
use lexer::*;

#[derive(Debug)]
pub enum Expr {
    Funcall { name: String, args: Vec<Expr> },
    StrLit { n: usize, len: usize },
    Intrinsic(Intrinsic)
}

#[derive(Debug)]
pub enum Intrinsic {
    Print
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub exprs: Vec<Expr>
}

fn parse_args(lexer: &mut Lexer, data: &mut Vec<String>) -> Vec<Expr>{
    let mut args = Vec::new();
    loop {
        if let Some(token) = lexer.next() {
            match token.kind {
                TokenKind::Litrl(Literal::Str(text)) => {
                    args.push(Expr::StrLit {n: data.len(), len: text.len() });
                    data.push(text);
                }
                TokenKind::Identifier(_) => {
                    todo!("identifiers as function args");
                }
                TokenKind::CloseParen => {
                    break;
                }
                TokenKind::Comma => {
                    continue;
                }
                token => logging::unexpected_token(TokenKind::CloseParen, token),
            }
        } else {
            logging::no_expected_token(TokenKind::CloseParen);
        }
    }
    args
}

fn parse_fn(lexer: &mut Lexer, data: &mut Vec<String>) -> Function {

    let mut exprs = Vec::new();
    let name;
    if let Some(Token { kind: TokenKind::Identifier(function_name) }) = lexer.next() { 
        name = function_name;
    } else {
        logging::syntax_err("Expected function name");
    }

    lexer.expect(TokenKind::OpenParen);
    //no args for now
    lexer.expect(TokenKind::CloseParen);

    lexer.expect(TokenKind::OpenCurly);

    while let Some(token) = lexer.next() {
        match token.kind {
            TokenKind::Identifier(text) => {
                lexer.expect(TokenKind::OpenParen);
                exprs.push(Expr::Funcall { name: text, args: parse_args(lexer, data) })
            }
            TokenKind::CloseCurly => {
                break;
            }
            TokenKind::Semicolon => (),
            token => logging::unexpected_token(TokenKind::CloseCurly, token)
        }
    }

    Function {
        name,
        exprs,
    }
}

fn stdlib() -> Vec<Function> {
    vec![
        Function {
            name: "print".to_string(),
            exprs: vec![Expr::Intrinsic(Intrinsic::Print)]
        }
    ]
}

pub fn parse(lexer: &mut Lexer) -> (Vec<Function>, Vec<String>) {

    let mut functions = stdlib();
    let mut data = Vec::new();

    while let Some(token) = lexer.next() {
        match token.kind {
            TokenKind::Kword(Keyword::Fn) => {
                functions.push(parse_fn(lexer, &mut data));
            }
            _ => {
                logging::syntax_err("Not allowed outside function definition");
            }
        }
    }

    (functions, data)
}
