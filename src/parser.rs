use super::*;
use lexer::*;

#[derive(Debug)]
pub enum Expr {
    Funcall { name: String, args: Vec<Expr> },
    StrLit { n: usize, len: usize },
    Intrinsic(Intrinsic),
    Var(String),
}

#[derive(Debug)]
pub enum Intrinsic {
    Print,
    PrintNum
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub exprs: Vec<Expr>
}

#[derive(Debug, Default)]
pub struct Data {
    pub strings: Vec<String>,
    pub globals: Vec<String>
}

fn parse_args(lexer: &mut Lexer, data: &mut Data) -> Vec<Expr>{
    let mut args = Vec::new();
    loop {
        if let Some(token) = lexer.next() {
            match token.kind {
                TokenKind::Litrl(Literal::Str(text)) => {
                    args.push(Expr::StrLit {n: data.strings.len(), len: text.len() });
                    data.strings.push(text);
                }
                TokenKind::Identifier(name) => {
                    if data.globals.contains(&name) {
                        args.push(Expr::Var(name))
                    } else {
                        logging::name_err(&name);
                    }
                }
                TokenKind::CloseParen => {
                    break;
                }
                TokenKind::Comma => {
                    continue;
                }
                _ => logging::unexpected_token(&lexer.file, TokenKind::CloseParen, token),
            }
        } else {
            logging::no_expected_token(&lexer.file, TokenKind::CloseParen);
        }
    }
    args
}

fn parse_fn(lexer: &mut Lexer, data: &mut Data) -> Function {

    let mut exprs = Vec::new();
    let name;
    let tok = lexer.next();
    if let Some(TokenKind::Identifier(fname)) = tok.map(|t| t.kind) {
        name = fname;
    } else {
        logging::syntax_err(&lexer.file, lexer.loc(), "Expected function name");
    }
    /*
    if let Some(token) = lexer.next() { 
        name = function_name;
    */

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
                return Function {
                    name,
                    exprs
                }
            }
            TokenKind::Semicolon => (),
            _ => logging::unexpected_token(&lexer.file, TokenKind::CloseCurly, token)
        }
    }

    logging::no_expected_token(&lexer.file, TokenKind::CloseCurly);
}

fn stdlib() -> Vec<Function> {
    vec![
        Function {
            name: "print".to_string(),
            exprs: vec![Expr::Intrinsic(Intrinsic::Print)]
        },
        Function {
            name: "print_num".to_string(),
            exprs: vec![Expr::Intrinsic(Intrinsic::PrintNum)]
        }
    ]
}

pub fn parse(lexer: &mut Lexer) -> (Vec<Function>, Data) {

    let mut functions = stdlib();
    let mut data = Data::default();

    while let Some(token) = lexer.next() {
        match token.kind {
            TokenKind::Kword(Keyword::Fn) => {
                functions.push(parse_fn(lexer, &mut data));
            }
            TokenKind::Kword(Keyword::Let) => {
                if let Some(TokenKind::Identifier(var_name)) = lexer.next().map(|t| t.kind) {
                    data.globals.push(var_name);
                }
            }
            TokenKind::Semicolon => (),
            _ => {
                logging::syntax_err(&lexer.file, lexer.loc(), "Not allowed outside function definition");
            }
        }
    }

    (functions, data)
}
